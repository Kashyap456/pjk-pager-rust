use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    headers::UserAgent,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router, TypedHeader,
};
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};

use futures::{sink::SinkExt, stream::select_all, stream::StreamExt};
use reqwest;
use tokio::sync::{broadcast, Mutex};
use tokio_stream::wrappers::BroadcastStream;

#[derive(Deserialize)]
struct User {
    name: String,
}

#[derive(Deserialize)]
struct GroupResponse {
    array: Vec<(String, i64)>,
}

type Groups = Arc<Mutex<HashMap<String, broadcast::Sender<String>>>>;

#[tokio::main]
async fn main() {
    let groups = reqwest::get("http://0.0.0.0:8000/groups").await.unwrap();
    let groups = groups.text().await.unwrap();
    eprintln!("{}", groups.as_str());
    let groups: Vec<String> = serde_json::from_str(groups.as_str()).unwrap();
    let mut connected_users: Groups = Arc::new(Mutex::new(HashMap::new()));
    for group in groups {
        let (tx, _rx) = broadcast::channel(100);
        connected_users.lock().await.insert(group, tx);
    }
    let app = Router::new()
        .route("/ws", get(handler))
        .with_state(connected_users);

    axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(ws: WebSocketUpgrade, State(groups): State<Groups>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, "kashyap".to_owned(), groups))
}

async fn handle_socket(mut socket: WebSocket, user: String, map: Groups) {
    let (mut sender, mut receiver) = socket.split();
    let mut receivers = Vec::new();
    let client = reqwest::Client::new();
    let groups = client
        .get("http://0.0.0.0:8000/userin")
        .query(&[("user", user)])
        .send()
        .await
        .unwrap();
    let groups = groups.text().await.unwrap();
    //eprintln!("{}", groups.as_str());
    let groups: Vec<(String, i64)> = serde_json::from_str(groups.as_str()).unwrap();
    for group in groups {
        let group = group.0;
        let guard = map.lock().await;
        if let Some(tx) = guard.get(&group) {
            let rx = tx.clone().subscribe();
            receivers.push(rx);
        }
    }
    let mut fused_streams = Arc::new(Mutex::new(select_all(
        receivers.into_iter().map(BroadcastStream::new),
    )));
    let send_streams = Arc::clone(&fused_streams);

    let mut send_task = tokio::spawn(async move {
        loop {
            let msg = {
                let mut guard = send_streams.lock().await;
                guard.next().await
            };
            match msg {
                Some(msg) => {
                    let msg = msg.unwrap();
                    eprintln!("{}", &msg);
                    if sender.send(Message::Text(msg)).await.is_err() {
                        break;
                    }
                }
                None => break,
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        let fused_streams = Arc::clone(&fused_streams);
        let map = Arc::clone(&map);
        while let Some(Ok(Message::Text(group))) = receiver.next().await {
            // Add username before message.
            eprintln!("{}", &group);
            let group: Vec<&str> = group.split_whitespace().collect();
            match group[0] {
                "page" => {
                    let group = group[1];
                    let gmap = map.lock().await;
                    if let Some(tx) = gmap.get(group) {
                        tx.send("GET ON LEAGUE OF LEGENDS".to_string());
                    }
                }
                "create" => {
                    let group = group[1];
                    let mut gmap = map.lock().await;
                    let (tx, _rx) = broadcast::channel(100);
                    gmap.insert(group.to_owned(), tx);
                }
                "join" => {
                    let group = group[1];
                    let mut gmap = map.lock().await;
                    let tx = gmap.get(group);
                    if let Some(txo) = tx {
                        let rx = txo.subscribe();
                        let mut select_all_guard = fused_streams.lock().await;
                        select_all_guard.push(BroadcastStream::new(rx));
                    }
                }
                _ => (),
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
