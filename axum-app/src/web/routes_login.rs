use serde_json::{Value, json};
use serde::Deserialize;
use axum::{Json, Router};
use crate::error::{Result, Error};
use axum::routing::post;
use tower_cookies::Cookies;
use crate::web;
use tower_cookies::Cookie;

use crate::rjwt::jwt::get_signture;

// route

pub fn routes() -> Router{
    Router::new()
        .route("/api/login", post(api_login))
}

#[derive(Debug, Deserialize)]
struct ApiLogin {
    username: String,
    pwd: String,
}


async fn api_login(cookie: Cookies ,payload: Json<ApiLogin>) -> Result<Json<Value>> {
    // Todo: implement the auth and db

    println!("->> {:<12} - api_login", "HANDLER");
    if payload.username.clone() != "demo" || payload.pwd != "lanmei" {
        return Err(Error::LoginFail);
    }

    //cookie
    let token_auth = get_signture(payload.username.clone());
    cookie.add(Cookie::new(web::AUTH_TOKEN, token_auth));

    let body = Json(json!(
        {
            "result": 
            {
                "success": true
            }
        }
    ));

    Ok(body)

}