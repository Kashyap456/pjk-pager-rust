use axum::{
    async_trait,
    extract::{self, FromRequest},
    http::{self, Request},
};
use sqlx::SqlitePool;

pub struct Conn(pub SqlitePool);

#[async_trait]
impl<S, B> FromRequest<S, B> for Conn
where
    // these bounds are required by `async_trait`
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = http::StatusCode;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let extract::Extension(pool) = extract::Extension::<SqlitePool>::from_request(req, state)
            .await
            .unwrap();
        Ok(Self(pool))
    }
}
