use core::panic;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use http::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tower_http::cors::{Any, CorsLayer};

const DB_URL: &str = "sqlite://sqlite.db";

mod db;
mod handlers;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database...");
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Database created successfully."),
            Err(error) => panic!("error: {}", error),
        }
    }

    let db = SqlitePool::connect(DB_URL).await.unwrap();

    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");

    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(&db)
        .await;

    match migration_results {
        Ok(_) => println!("Migration complete."),
        Err(error) => panic!("error: {}", error),
    }

    println!("migration: {:?}", migration_results);
    //.route_layer(axum::middleware::from_fn(handlers::check_user_auth))
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/groups",
            post(handlers::create_group).get(handlers::list_groups),
        )
        .route(
            "/memberships",
            get(handlers::list_memberships).post(handlers::join_group),
        )
        .route("/users", post(handlers::sync_user))
        .route("/userin", get(handlers::list_memberships_by_user))
        .layer(Extension(db))
        .layer(CorsLayer::very_permissive());

    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
