use serde::{Serialize, Deserialize};
#[derive(Debug, Deserialize)]
pub struct InMessage{
    pub room: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct OutMessage{
    pub user_id: String,
    pub message: String,
    pub datetime: chrono::DateTime<chrono::Utc>,
}
