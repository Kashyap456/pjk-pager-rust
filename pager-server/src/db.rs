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
        group_owner,
        is_admin
    )
    .execute(&pool)
    .await
    .unwrap();
}
