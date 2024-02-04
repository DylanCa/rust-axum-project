use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize)]
pub struct InMessage {
    pub room: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct OutMessage {
    pub user_id: String,
    pub message: String,
    pub datetime: chrono::DateTime<chrono::Utc>,
#[derive(Debug, sqlx::FromRow)]
pub struct Chatroom {
    id: String,
    name: String,
    created_at: DateTime<Utc>,
}
impl Chatroom {
    pub fn new(name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            created_at: chrono::Utc::now(),
        }
    }
}
}
