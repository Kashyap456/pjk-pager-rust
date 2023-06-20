use core::panic;

use axum::{routing::get, Extension, Router};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tower_cookies::CookieManagerLayer;

mod db;
mod handlers;

const DB_URL: &str = "sqlite://keyvalue.db";

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    std::env::set_var("DATABASE_URL", DB_URL);

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

    let app = Router::new()
        .layer(axum::middleware::from_fn(handlers::auth_fn))
        .route("/", get(|| async { "Hello, World!" }))
        .route("/register_client", get(handlers::register_client))
        .layer(Extension(db))
        .layer(CookieManagerLayer::new());

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
