use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use sqlx::{Pool, Sqlite};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub mod handlers {

    #[derive(Hash)]
    struct Identity {
        id: u32,
        secret: String,
    }

    pub async fn register_client(pool: Pool<Sqlite>) -> Result<(u32, String), StatusCode> {
        let mut hasher = DefaultHasher::new();
        let mut rng = thread_rng();
        let client_id: u32 = rng.gen();
        let client_secret: String = rng
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();
        let identity = Identity(client_id, client_secret);
        let hash_value = identity.hash(&mut hasher);
        sqlx::query!("INSERT INTO keys VALUES (?, ?)", client_id, hash_value)
            .fetch(&pool)
            .await?;

        Ok(client_id, client_secret)
    }
}
