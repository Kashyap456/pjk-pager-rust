use crate::db::{self, UserRec};
use axum::extract::{Extension, TypedHeader};
use axum::headers::{authorization::Bearer, Authorization};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::{Json, RequestPartsExt};
use axum_macros::debug_handler;
use cookie::time::Duration;
use http::{Request, StatusCode};
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
        return Err(StatusCode::UNAUTHORIZED);
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
        return Err(StatusCode::UNAUTHORIZED);
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
    let mut access_cookie = Cookie::new(access_token.clone(), "auth");
    access_cookie.set_max_age(Some(Duration::DAY));
    cookies.add(access_cookie);
    let mut refresh_cookie = Cookie::new(refresh_token.clone(), "refresh");
    refresh_cookie.set_max_age(Some(Duration::WEEK));
    cookies.add(refresh_cookie);
    Ok(Json(
        json!({"access_token": access_token, "refresh_token": refresh_token}),
    ))
}

pub async fn auth_fn<B>(
    cookies: Cookies,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode>
where
    B: Send,
{
    // running extractors requires a `axum::http::request::Parts`
    let (mut parts, body) = request.into_parts();

    // `TypedHeader<Authorization<Bearer>>` extracts the auth token
    let auth: TypedHeader<Authorization<Bearer>> = parts
        .extract()
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let client_cookie = cookies.get(auth.token());
    if client_cookie == None {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // reconstruct the request
    let request = Request::from_parts(parts, body);

    Ok(next.run(request).await)
}

pub async fn resource_auth<B>(
    cookies: Cookies,
    request: Request<B>,
) -> Result<StatusCode, StatusCode>
where
    B: Send,
{
    // running extractors requires a `axum::http::request::Parts`
    let (mut parts, body) = request.into_parts();

    // `TypedHeader<Authorization<Bearer>>` extracts the auth token
    let auth: TypedHeader<Authorization<Bearer>> = parts
        .extract()
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let client_cookie = cookies.get(auth.token());
    if client_cookie == None {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(StatusCode::OK)
}
