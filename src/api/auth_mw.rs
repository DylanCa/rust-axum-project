use crate::api::AUTH_TOKEN;
use crate::ctx::Ctx;
use crate::Error;
use async_trait::async_trait;
use axum::body::Body;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestPartsExt;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

pub async fn auth_required<B>(
    ctx: Result<Ctx, Error>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, Error> {
    ctx?;

    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Error> {
        let cookies = parts.extract::<Cookies>().await.unwrap();

        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        let (user_id, _exp, _sign) = auth_token
            .ok_or(Error::AuthFailNoAuthTokenCookie)
            .and_then(parse_token)?;

        // FIXME: Add exp & sign checks here

        Ok(Ctx::new(user_id))
    }
}

fn parse_token(token: String) -> Result<(String, String, String), Error> {
    let (_whole, user_id, exp, sign) =
        regex_captures!(r#"^(.+)\.(.+)\.(.+)"#, &token).ok_or(Error::AuthFailTokenWrongFormat)?;

    Ok((user_id.to_string(), exp.to_string(), sign.to_string()))
}
