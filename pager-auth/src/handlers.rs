use crate::db;
use crate::params::Conn;
use axum::response::IntoResponse;
use axum::Json;
use axum_macros::debug_handler;
use http::StatusCode;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rand::{rngs::StdRng, SeedableRng};
use serde_json::json;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Hash)]
struct Identity {
    id: u32,
    secret: String,
}

#[debug_handler]
pub async fn register_client(Conn(conn): Conn) -> Result<impl IntoResponse, StatusCode> {
    let mut hasher = DefaultHasher::new();
    let mut rng = StdRng::seed_from_u64(42);
    let client_id: u32 = rng.gen();
    let client_secret: String = rng
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    let identity = Identity {
        id: client_id,
        secret: client_secret.clone(),
    };
    identity.hash(&mut hasher);
    let hash_value = hasher.finish();
    db::add_client(conn, client_id.to_string(), hash_value.to_string()).await;
    Ok(Json(
        json!({"client_id": client_id, "client_secret": client_secret}),
    ))
}
