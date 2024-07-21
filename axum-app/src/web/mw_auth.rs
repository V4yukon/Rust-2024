use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;
use axum::body::{Body, HttpBody};

use crate::web::AUTH_TOKEN;
use crate::error::{Error, Result};

pub async  fn mw_require_auth(
    cookie: Cookies,
    req: Request<Body>,
    next: Next
) -> Result<Response> 
{
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");
    let auth_token = cookie.get(AUTH_TOKEN).map(|c| c.value().to_string());

    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;
    Ok(next.run(req).await)
}

// ToDo  parse 