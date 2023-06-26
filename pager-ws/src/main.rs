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

use futures::{sink::SinkExt, stream::StreamExt};
use reqwest;
use tokio::sync::broadcast;

#[derive(Deserialize)]
struct User {
    name: String,
}

type Clients = Arc<Mutex<HashMap<String, broadcast::Sender<String>>>>;

#[tokio::main]
async fn main() {
    //let groups = reqwest::get("0.0.0.0:3000/groups").await.unwrap();
    //print!("{}", groups.text().await.unwrap());
    let mut connected_users: Clients = Arc::new(Mutex::new(HashMap::new()));
    let app = Router::new()
        .route("/ws", get(handler))
        .with_state(connected_users);

    axum::Server::bind(&"0.0.0.0:7777".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket))
}

async fn handle_socket(mut socket: WebSocket) {
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
