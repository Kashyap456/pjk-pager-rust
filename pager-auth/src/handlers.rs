use crate::db::{self, UserRec};
use axum::extract::Extension;
use axum::response::IntoResponse;
use axum::Json;
use axum_macros::debug_handler;
use http::StatusCode;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rand::{rngs::StdRng, SeedableRng};
use serde::Deserialize;
use serde_json::json;
use sqlx::{Pool, Sqlite};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use tower_cookies::{Cookie, Cookies};

#[derive(Hash)]
struct Identity {
    id: u32,
    secret: String,
}

#[derive(Hash, Deserialize)]
pub struct User {
    username: String,
    password: String,
}

#[debug_handler]
pub async fn register_client(
    Extension(conn): Extension<Pool<Sqlite>>,
    cookies: Cookies,
) -> Result<impl IntoResponse, StatusCode> {
    let mut hasher = DefaultHasher::new();
    let mut rng = StdRng::from_entropy();
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
    let client_cookie = cookies.get(client_id.to_string().clone().as_str());
    if client_cookie == None {
        let name = client_id.to_string();
        cookies.add(Cookie::new(name, "authenticated"))
    }
    Ok(Json(
        json!({"client_id": client_id, "client_secret": client_secret}),
    ))
}

#[debug_handler]
pub async fn register_user(
    Extension(conn): Extension<Pool<Sqlite>>,
    cookies: Cookies,
    Json(body): Json<User>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut hasher = DefaultHasher::new();
    let mut rng = StdRng::from_entropy();
    let salt: String = rng
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    let saltpass = body.password + salt.as_str();
    let user = User {
        username: body.username.clone(),
        password: saltpass,
    };
    user.hash(&mut hasher);
    let hashed_val = hasher.finish();
    db::add_user(conn, body.username.clone(), salt, hashed_val.to_string()).await;
    Ok(Json(json!("Successful user creation.")))
}

#[debug_handler]
pub async fn login_user(
    Extension(conn): Extension<Pool<Sqlite>>,
    cookies: Cookies,
    Json(body): Json<User>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut hasher = DefaultHasher::new();
    let rec = db::get_user(conn.clone(), body.username.clone()).await;
    if Option::is_none(&rec) {
        return Err(StatusCode::FORBIDDEN);
    }
    let mut rng = StdRng::from_entropy();
    let rec = rec.unwrap();
    let saltpass = body.password + rec.salt.as_str();
    let user = User {
        username: body.username.clone(),
        password: saltpass,
    };
    user.hash(&mut hasher);
    let hashed_val = hasher.finish();
    if rec.userhash != hashed_val.to_string() {
        return Err(StatusCode::FORBIDDEN);
    }
    let access_token: String = rng
        .clone()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    let refresh_token: String = rng
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();
    cookies.add(Cookie::new(
        body.username.clone() + "_access",
        access_token.clone(),
    ));
    cookies.add(Cookie::new(
        body.username.clone() + "_refresh",
        refresh_token.clone(),
    ));
    Ok(Json(
        json!({"access_token": access_token, "refresh_token": refresh_token}),
    ))
}
