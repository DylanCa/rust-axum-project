use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Note {
    #[serde(skip_deserializing)]
    id: String,

    #[serde(skip_deserializing)]
    user_id: String,
    pub title: String,
    pub content: String,

    #[serde(skip_deserializing)]
    created_at: Option<DateTime<Utc>>,

    #[serde(skip_deserializing)]
    updated_at: Option<DateTime<Utc>>,
}

impl Note {
    pub fn new(user_id: String, title: String, content: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id,
            title,
            content,
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn user_id(&self) -> &String {
        &self.user_id
    }
}

#[derive(Debug, Serialize)]
pub struct NoteResponse {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
