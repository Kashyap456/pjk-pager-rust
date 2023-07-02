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

use futures::{
    sink::SinkExt,
    stream::select_all,
    stream::{SelectAll, StreamExt},
};
use reqwest;
use tokio::sync::{broadcast, Mutex};
use tokio_stream::wrappers::BroadcastStream;

#[derive(Deserialize)]
struct User {
    name: String,
}

#[derive(Deserialize)]
struct GroupResponse {
    array: Vec<String>,
}
#[derive(Clone)]
struct AppState {
    groups: Groups,
}

impl AppState {
    async fn get_group(&mut self, group: String) -> Option<broadcast::Sender<String>> {
        self.groups.lock().await.get(&group).cloned()
    }

    async fn set_group(&mut self, group: String, channel: broadcast::Sender<String>) {
        self.groups.lock().await.insert(group, channel);
    }
}

type Groups = Arc<Mutex<HashMap<String, broadcast::Sender<String>>>>;

#[tokio::main]
async fn main() {
    let groups = reqwest::get("http://0.0.0.0:3000/groups").await.unwrap();
    let groups = groups.text().await.unwrap();
    let groups: Vec<String> = serde_json::from_str(groups.as_str()).unwrap();
    let mut connected_users: Groups = Arc::new(Mutex::new(HashMap::new()));
    let mut app_state = AppState {
        groups: connected_users,
    };
    for group in groups {
        let (tx, _rx) = broadcast::channel(100);
        app_state.set_group(group, tx);
    }
    let app = Router::new()
        .route("/ws", get(handler))
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:7777".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

//TODO: unhardcode this
async fn handler(ws: WebSocketUpgrade, State(groups): State<AppState>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, "kashyap".to_owned(), groups))
}

async fn handle_socket(mut socket: WebSocket, user: String, mut map: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let mut receivers = Vec::new();
    let client = reqwest::Client::new();
    let groups = client
        .get("http://0.0.0.0:3000/userin")
        .query(&[("user", user)])
        .send()
        .await
        .unwrap();
    let groups = groups.text().await.unwrap();
    //eprintln!("{}", groups.as_str());
    let groups: Vec<String> = serde_json::from_str(groups.as_str()).unwrap();
    for group in groups {
        if let Some(tx) = map.get_group(group).await {
            let rx = tx.clone().subscribe();
            receivers.push(rx);
        }
    }

    let mut fused_streams = Arc::new(Mutex::new(select_all(
        receivers.into_iter().map(BroadcastStream::new),
    )));
    let mut send_stream = fused_streams.clone();

    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = fused_streams.lock().await.next().await {
            // In any websocket error, break loop.
            let msg = msg.unwrap();
            eprintln!("{}", &msg);
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(command))) = receiver.next().await {
            // Add username before message.
            eprintln!("{}", &command);
            let cmd_vec: Vec<&str> = command.split_whitespace().collect();
            match cmd_vec[0] {
                "page" => {
                    let group = cmd_vec[1];
                    if let Some(tx) = map.get_group(group.to_owned()).await {
                        tx.send("GET ON LEAGUE OF LEGENDS".to_string());
                    }
                }
                "create" => {
                    let group = cmd_vec[1];
                    let (tx, rx) = broadcast::channel(100);
                    let rx = tx.clone().subscribe();
                    map.set_group(group.to_owned(), tx);
                    send_stream.lock().await.push(rx.into());
                }
                _ => eprintln!("Ill-formatted command"),
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
