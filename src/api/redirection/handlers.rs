use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};
use std::sync::Arc;
use rand::prelude::SliceRandom;
use sqlx::Row;

use crate::api::redirection::models::{Redirection, RedirectionShortcode, RedirectionParams};
use crate::AppState;
use crate::ctx::Ctx;

pub async fn create_redirection(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(payload): Json<Redirection>,
) -> Result<(StatusCode, Json<Redirection>), (StatusCode, Json<Value>)> {

    let query_result = sqlx::query(r#"SELECT * FROM redirections WHERE url = ?"#)
        .bind(payload.url.clone())
        .fetch_one(&state.db)
        .await;

    let redirect;
    match query_result {
        Ok(r) => redirect = Redirection::new(r.get("shortcode"), payload.url, ctx.user_id().clone()),
        Err(_) => {
            let content = include_str!("words.txt");
            let words: Vec<&str> = content.lines().collect();
            let mut shortcode: Vec<&str> = vec!();
            for _ in 0..3 {
                shortcode.push(words.choose(&mut rand::thread_rng()).unwrap());
            }
            let r = Redirection::new(shortcode.join("-"), payload.url, ctx.user_id().clone());

            sqlx::query(r#"INSERT INTO redirections (id, user_id, shortcode, url, utilizations) VALUES (?, ?, ?, ?, ?)"#)
                .bind(r.id().clone())
                .bind(ctx.user_id().clone())
                .bind(r.shortcode.clone())
                .bind(r.url.clone())
                .bind(r.utilizations().clone())
                .execute(&state.db)
                .await
                .unwrap();

            redirect = r;

        }
    }

    Ok((StatusCode::OK, Json(redirect)))
}

pub async fn get_redirection_url(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Query(code): Query<RedirectionParams>,
) -> Result<(StatusCode, Json<RedirectionShortcode>), (StatusCode, Json<Value>)> {
        let query_result = sqlx::query_as!(
            RedirectionShortcode,
            r#"SELECT * FROM redirections WHERE shortcode = ?"#,
            code.code.unwrap()
        )
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