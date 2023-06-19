use sqlx::SqlitePool;

pub struct UserRec {
    username: String,
    pub salt: String,
    pub userhash: String,
}

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

pub async fn add_user(pool: SqlitePool, user_id: String, salt: String, hash_value: String) {
    sqlx::query!(
        "INSERT INTO users (username, salt, userhash) VALUES (?, ?, ?)",
        user_id,
        salt,
        hash_value
    )
    .execute(&pool)
    .await
    .unwrap();
}

pub async fn get_user(pool: SqlitePool, user_id: String) -> Option<UserRec> {
    let rec = sqlx::query!(
        "SELECT username, salt, userhash from users WHERE username = ?",
        user_id
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    if let Some(record) = rec {
        Some(UserRec {
            username: record.username,
            salt: record.salt,
            userhash: record.userhash,
        })
    } else {
        None
    }
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
