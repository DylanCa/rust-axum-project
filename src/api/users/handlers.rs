use crate::api::users::models::{User, UserResponse};
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde_json::json;
use std::sync::Arc;

pub async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    State(data): State<Arc<AppState>>,
    Json(payload): Json<User>,
) -> Result<(StatusCode, Json<User>), (StatusCode, Json<serde_json::Value>)> {
    let user = User::new(payload);

    let query_result = sqlx::query(r#"INSERT INTO users (id, name, email, password, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)"#)
        .bind(user.id().clone())
        .bind(user.name.clone())
        .bind(user.email().clone())
        .bind(user.password_hash().clone())
        .bind(user.created_at().clone())
        .bind(user.updated_at().clone())
        .execute(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    println!("{:#?}", query_result);

    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            let error_response = serde_json::json!({
                "status": "error",
                "message": "User already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", err)})),
        ));
    }

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Path(uuid): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<UserResponse>), (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        UserResponse,
        r#"SELECT * FROM users WHERE id = ?"#,
        uuid.to_string()
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", e)})),
        )
    })?;

    println!("{:#?}", query_result);

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    Ok((StatusCode::CREATED, Json(query_result)))
}
