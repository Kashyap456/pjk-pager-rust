use crate::db;
use axum::extract::{Extension, TypedHeader};
use axum::headers::{authorization::Bearer, Authorization};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::{Json, RequestPartsExt};
use http::{Request, StatusCode};
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use sqlx::{Pool, Sqlite};

#[derive(Deserialize)]
pub struct Group {
    name: String,
    user: String,
    members: Option<Vec<String>>,
}
#[derive(Deserialize)]
pub struct Membership {
    name: String,
    user: String,
    is_admin: Option<bool>,
}

pub async fn check_user_auth<B>(request: Request<B>, next: Next<B>) -> Result<Response, StatusCode>
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

    let client = reqwest::Client::new();
    let res = client
        .get("0.0.0.0:8080/auth_resource")
        .header(AUTHORIZATION, "Bearer ".to_owned() + auth.token())
        .send()
        .await
        .unwrap();

    if res.status() != StatusCode::OK {
        return Err(res.status());
    }

    // reconstruct the request
    let request = Request::from_parts(parts, body);

    Ok(next.run(request).await)
}

pub async fn create_group(
    Extension(conn): Extension<Pool<Sqlite>>,
    Json(body): Json<Group>,
) -> Result<impl IntoResponse, StatusCode> {
    db::add_group(conn.clone(), body.name.clone(), body.user.clone()).await;
    db::add_memberships(conn, body.user, body.name, 1).await;
    Ok(())
}

pub async fn join_group(
    Extension(conn): Extension<Pool<Sqlite>>,
    Json(body): Json<Membership>,
) -> Result<impl IntoResponse, StatusCode> {
    db::add_memberships(conn, body.user, body.name, 0).await;
    Ok(())
}

pub async fn sync_user(
    Extension(conn): Extension<Pool<Sqlite>>,
    Json(body): Json<Membership>,
) -> Result<impl IntoResponse, StatusCode> {
    db::sync_user(conn, body.user.clone()).await;
    Ok(body.user)
}

pub async fn list_groups(
    Extension(conn): Extension<Pool<Sqlite>>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = db::get_groups(conn).await;
    Ok(Json(res))
}

pub async fn list_memberships(
    Extension(conn): Extension<Pool<Sqlite>>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = db::get_groups(conn).await;
    Ok(Json(res))
}
