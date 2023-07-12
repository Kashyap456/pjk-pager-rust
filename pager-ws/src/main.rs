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
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use futures::{sink::SinkExt, stream::select_all, stream::StreamExt};
use reqwest;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

#[derive(Deserialize)]
struct User {
    name: String,
}

#[derive(Deserialize)]
struct GroupResponse {
    array: Vec<String>,
}

type Groups = Arc<Mutex<HashMap<String, broadcast::Sender<String>>>>;

#[tokio::main]
async fn main() {
    let groups = reqwest::get("http://127.0.0.1:8000/groups").await.unwrap();
    let groups = groups.text().await.unwrap();
    let groups: Vec<String> = serde_json::from_str(groups.as_str()).unwrap();
    let mut connected_users: Groups = Arc::new(Mutex::new(HashMap::new()));
    for group in groups {
        let (tx, _rx) = broadcast::channel(100);
        connected_users.lock().unwrap().insert(group, tx);
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
        .get("http://127.0.0.1:8000/userin")
        .query(&[("user", user)])
        .send()
        .await
        .unwrap();
    let groups = groups.text().await.unwrap();
    //eprintln!("{}", groups.as_str());
    let groups: Vec<String> = serde_json::from_str(groups.as_str()).unwrap();
    for group in groups {
        let guard = map.lock().unwrap();
        if let Some(tx) = guard.get(&group) {
            let rx = tx.clone().subscribe();
            receivers.push(rx);
        }
    }
    let mut fused_streams = select_all(receivers.into_iter().map(BroadcastStream::new));

    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = fused_streams.next().await {
            // In any websocket error, break loop.
            let msg = msg.unwrap();
            eprintln!("{}", &msg);
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(group))) = receiver.next().await {
            // Add username before message.
            eprintln!("{}", &group);
            let group: Vec<&str> = group.split_whitespace().collect();
            let group = group[1];
            let gmap = map.lock().unwrap();
            if let Some(tx) = gmap.get(group) {
                tx.send("GET ON LEAGUE OF LEGENDS".to_string());
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
