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

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct ChatMessage {
    id: String,
    room_id: String,
    user_id: String,
    message: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ChatToken {
    id: String,
    user_id: String,
    token: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ChatSession {
    id: String,
    socket_id: String,
    token_id: String,
    created_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
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

impl ChatMessage {
    pub fn new(room_id: String, user_id: String, message: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            room_id,
            user_id,
            message,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn room_id(self) -> String {
        self.room_id
    }
}

impl ChatToken {
    pub fn new(user_id: String, token: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id,
            token,
            created_at: chrono::Utc::now(),
        }
    }
}

impl ChatSession {
    pub fn new(socket_id: String, token_id: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            socket_id,
            token_id,
            created_at: chrono::Utc::now(),
            deleted_at: None,
        }
    }
}
