use bcrypt::{hash, DEFAULT_COST};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    #[serde(skip_deserializing)]
    id: String,
    pub name: String,
    email: String,
    password: String,

    #[serde(skip_deserializing)]
    created_at: Option<DateTime<Utc>>,

    #[serde(skip_deserializing)]
    updated_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(payload: User) -> Self {
        let hashed_password = hash(payload.password, DEFAULT_COST).unwrap();

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            password: hashed_password,
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
            ..payload
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }
    pub fn email(&self) -> &String {
        &self.email
    }
    pub fn password_hash(&self) -> &String {
        &self.password
    }
    pub fn created_at(&self) -> Option<DateTime<Utc>> {
        self.created_at
    }
    pub fn updated_at(&self) -> Option<DateTime<Utc>> {
        self.updated_at
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
