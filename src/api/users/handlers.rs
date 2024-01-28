use crate::api::users::models::{User, UserLogin, UserResponse};
use crate::api::AUTH_TOKEN;
use crate::config::connection::AppState;
use crate::ctx::Ctx;
use crate::Error::{LoginFailed, UserNotFound};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use bcrypt::verify;
use serde_json::json;
use std::sync::Arc;
use tower_cookies::{Cookie, Cookies};

pub async fn create_user(
    State(state): State<Arc<AppState>>,
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
        .execute(&state.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            let error_response = json!({
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
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
) -> Result<(StatusCode, Json<UserResponse>), (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        UserResponse,
        r#"SELECT * FROM users WHERE id = ?"#,
        ctx.user_id()
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_e| UserNotFound.into_code_value())?;

    Ok((StatusCode::OK, Json(query_result)))
}

pub async fn login(
    cookies: Cookies,
    State(data): State<Arc<AppState>>,
    Json(payload): Json<UserLogin>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let user = sqlx::query_as!(
        UserResponse,
        r#"SELECT * FROM users WHERE email = ?"#,
        payload.email
    )
    .fetch_one(&data.db)
    .await
    .map_err(|_e| LoginFailed.into_code_value())?;

    if !verify(payload.password, &*user.password).unwrap() {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"status": "error","message": "Wrong Password."})),
        ));
    }

    // FIXME: Implement a auth-token generation / exp / signature
    cookies.add(Cookie::new(
        AUTH_TOKEN,
        format!("{}.{}.{}", user.id, "exp", "sign"),
    ));

    Ok(StatusCode::OK)
}
