use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum Error {
    LoginFailed,
    WrongPassword,
    UserNotFound,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("{self:#?}");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
