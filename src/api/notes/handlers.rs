use std::sync::Arc;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};
use crate::api::notes::models::{Note, NoteResponse};
use crate::AppState;
use crate::ctx::Ctx;

pub async fn create_note(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(payload): Json<Note>,
) -> Result<(StatusCode, Json<Note>), (StatusCode, Json<Value>)> {
    let note = Note::new(ctx.user_id().to_owned(), payload.title, payload.content);

    let query_result = sqlx::query(r#"INSERT INTO notes (id, user_id, title, content) VALUES (?, ?, ?, ?)"#)
        .bind(note.id().clone())
        .bind(note.user_id().clone())
        .bind(note.title.clone())
        .bind(note.content.clone())
        .execute(&state.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", err)})),
        ));
    }

    Ok((StatusCode::CREATED, Json(note)))
}

pub async fn get_note(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Path(id): Path<String>
) -> Result<(StatusCode, Json<NoteResponse>), (StatusCode, Json<Value>)> {
    println!("{id:?}");
    let query_result = sqlx::query_as!(NoteResponse, r#"SELECT * FROM notes WHERE id = ? AND user_id = ?"#, id, ctx.user_id())
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            )
        })?;

    Ok((StatusCode::OK, Json(query_result)))
}