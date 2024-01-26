use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Redirection {
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    id: String,

    #[serde(skip_deserializing)]
    pub shortcode: String,

    pub url: String,

    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    user_id: String,

    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    utilizations: i32,

    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    created_at: Option<DateTime<Utc>>,

    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    updated_at: Option<DateTime<Utc>>,
}

impl Redirection {
    pub fn new(shortcode: String, url: String, user_id: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            shortcode,
            url,
            user_id,
            utilizations: 0,
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn utilizations(&self) -> &i32 {
        &self.utilizations
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct RedirectionShortcode {
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    pub id: String,
    pub shortcode: String,
    #[serde(skip_deserializing)]
    pub url: String,
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    pub user_id: String,
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    pub utilizations: i32,
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct RedirectionParams {
    pub code: Option<String>,
}
