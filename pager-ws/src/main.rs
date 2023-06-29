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
    let groups = reqwest::get("http://0.0.0.0:3000/groups").await.unwrap();
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

    axum::Server::bind(&"0.0.0.0:7777".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(
    ws: WebSocketUpgrade,
    State(groups): State<Groups>,
    Json(user): Json<User>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, user.name, groups))
}

async fn handle_socket(mut socket: WebSocket, user: String, map: Groups) {
    let mut receivers = Vec::new();
    let client = reqwest::Client::new();
    let groups = client
        .get("http://0.0.0.0:3000/userin")
        .query(&[("name", user)])
        .send()
        .await
        .unwrap();
    let groups = groups.text().await.unwrap();
    let groups: Vec<String> = serde_json::from_str(groups.as_str()).unwrap();
    for group in groups {
        let guard = map.lock().unwrap();
        if let Some(tx) = guard.get(&group) {
            let rx = tx.clone().subscribe();
            receivers.push(rx);
        }
    }
    let mut fused_streams = select_all(receivers.into_iter().map(BroadcastStream::new));
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };

        eprintln!("{}", msg.to_text().unwrap());
        if socket.send(msg).await.is_err() {
            // client disconnected
            return;
        }
    }
}
