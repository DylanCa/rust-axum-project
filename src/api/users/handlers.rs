use axum::Json;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

pub async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

pub async fn get_user() -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: String::from("Bob"),
    };

    (StatusCode::OK, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}
