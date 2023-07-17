use axum::Error;
use sqlx::SqlitePool;

pub async fn add_group(pool: SqlitePool, group_name: String, group_owner: String) {
    let out = sqlx::query!(
        "INSERT INTO groups (group_name, group_owner) VALUES (?, ?)",
        group_name,
        group_owner,
    )
    .fetch_one(&pool)
    .await;
    match out {
        Ok(_) => (),
        Err(error) => eprintln!("{}", error),
    };
    ()
}

pub async fn delete_group(pool: SqlitePool, group_name: String, group_owner: String) {
    let out: Result<_, sqlx::Error> = sqlx::query!(
        "DELETE FROM groups WHERE group_name = ? AND group_owner = ?",
        group_name,
        group_owner
    )
    .execute(&pool)
    .await;
    ()
}

pub async fn add_memberships(pool: SqlitePool, user: String, group_name: String, is_admin: u32) {
    eprintln!("{} {}", user, group_name);
    sqlx::query!(
        "INSERT INTO memberships (user, group_name, is_admin) VALUES (?, ?, ?)",
        user,
        group_name,
        is_admin
    )
    .execute(&pool)
    .await
    .unwrap();
}

pub async fn delete_memberships(pool: SqlitePool, group_name: String) {
    sqlx::query!("DELETE FROM memberships WHERE group_name = ?", group_name)
        .execute(&pool)
        .await
        .unwrap();
}

pub async fn get_groups(pool: SqlitePool) -> Vec<String> {
    let mut vec = Vec::new();
    let records = sqlx::query!("SELECT group_name as name FROM groups")
        .fetch_all(&pool)
        .await
        .unwrap();
    for record in records {
        vec.push(record.name.unwrap());
    }
    vec
}

pub async fn get_memberships(pool: SqlitePool) -> Vec<(String, String)> {
    let mut vec = Vec::new();
    let records = sqlx::query!("SELECT user, group_name as name FROM memberships")
        .fetch_all(&pool)
        .await
        .unwrap();
    for record in records {
        vec.push((record.user, record.name));
    }
    vec
}

pub async fn get_memberships_by_user(pool: SqlitePool, user: String) -> Vec<(String, i64)> {
    let mut vec = Vec::new();
    let records = sqlx::query!(
        "SELECT user, is_admin, group_name as name FROM memberships WHERE user = ?",
        user
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    for record in records {
        let admin = record.is_admin;
        let admin: i64 = admin.unwrap_or(0);
        vec.push((record.name, admin));
    }
    vec
}

pub async fn sync_user(pool: SqlitePool, user: String) {
    sqlx::query!("INSERT OR REPLACE INTO users (username) VALUES (?)", user)
        .execute(&pool)
        .await
        .unwrap();
}
