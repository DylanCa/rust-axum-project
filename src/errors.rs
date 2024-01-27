use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use log::info;
use serde_json::{json, Value};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFailed,
    WrongPassword,
    UserNotFound,
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    UrlNotFound,
}

impl Error {
    pub fn into_code_value(self) -> (StatusCode, Json<Value>) {
        info!("{self:#?}");

        let mut response = (
            StatusCode::INTERNAL_SERVER_ERROR,
            false,
            "UNHANDLED_CLIENT_ERROR",
        );
        match self {
            Error::LoginFailed
            | Error::WrongPassword
            | Error::UserNotFound
            | Error::AuthFailTokenWrongFormat => {
                response = (StatusCode::UNAUTHORIZED, false, "LOGIN_FAILED")
            }
            Error::AuthFailNoAuthTokenCookie => {
                response = (StatusCode::UNAUTHORIZED, false, "NOT_LOGGED_IN")
            }
            Error::UrlNotFound => response = (StatusCode::NOT_FOUND, false, "URL_NOT_FOUND"),
        }

        let status_code = response.0;
        let body = json!({"response": {"success": response.1, "message": response.2}});
        (status_code, Json(body))
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        self.into_code_value().into_response()
    }
}
