use sqlx::SqlitePool;

pub async fn add_group(pool: SqlitePool, group_name: String, group_owner: String) {
    sqlx::query!(
        "INSERT INTO groups (group_name, group_owner) VALUES (?, ?)",
        group_name,
        group_owner,
    )
    .execute(&pool)
    .await
    .unwrap();
}

pub async fn add_memberships(pool: SqlitePool, user: String, group_name: String, is_admin: u32) {
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

pub async fn get_memberships_by_user(pool: SqlitePool, user: String) -> Vec<String> {
    let mut vec = Vec::new();
    let records = sqlx::query!(
        "SELECT user, group_name as name FROM memberships WHERE user = ?",
        user
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    for record in records {
        vec.push(record.name);
    }
    vec
}

pub async fn sync_user(pool: SqlitePool, user: String) {
    sqlx::query!("INSERT OR REPLACE INTO users (username) VALUES (?)", user)
        .execute(&pool)
        .await
        .unwrap();
}
