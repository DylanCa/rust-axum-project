use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFailed,
    WrongPassword,
    UserNotFound,
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("{self:#?}");

        let mut response = (StatusCode::INTERNAL_SERVER_ERROR, false, "UNHANDLED_CLIENT_ERROR");
        match self {
            Error::LoginFailed | Error::WrongPassword | Error::UserNotFound | Error::AuthFailNoAuthTokenCookie | Error::AuthFailTokenWrongFormat => response = (StatusCode::UNAUTHORIZED, false, "LOGIN_FAILED")
        }

        let status_code = response.0;
        let body = json!({"response": {"success": response.1, "message": response.2}});
        (status_code, Json(body)).into_response()
    }
}
