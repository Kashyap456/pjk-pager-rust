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
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};

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

    //let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or("pager-auth/".to_owned());
    let migrations = std::path::Path::new("./migrations");

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
        .route("/auth_resource", get(handlers::resource_auth))
        .route("/login_user", post(handlers::login_user))
        .route("/register_user", post(handlers::register_user))
        .layer(Extension(db))
        .layer(CookieManagerLayer::new())
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_origin(Any)
                .allow_headers([AUTHORIZATION, CONTENT_TYPE]),
        );

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
