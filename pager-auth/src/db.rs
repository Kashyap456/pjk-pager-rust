use sqlx::SqlitePool;

pub async fn add_client(pool: SqlitePool, client_id: String, hash_value: String) {
    sqlx::query!(
        "INSERT INTO keyval (ukey, hashpass) VALUES (?, ?)",
        client_id,
        hash_value
    )
    .execute(&pool)
    .await
    .unwrap();
}

pub async fn verify_client(pool: SqlitePool, client_id: String, hash_value: String) -> bool {
    let record = sqlx::query!("SELECT hashpass FROM keyval WHERE ukey = ?", client_id)
        .fetch_optional(&pool)
        .await
        .unwrap();

    if let Some(rec) = record {
        if let Some(pass) = rec.hashpass {
            return pass == hash_value;
        }
    }
    false
}
