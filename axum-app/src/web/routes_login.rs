use serde_json::{Value, json};
use serde::Deserialize;
use axum::{Json, Router};
use crate::error::{Result, Error};
use axum::routing::post;


// route

pub   fn routes() -> Router{
    Router::new()
        .route("/api/login", post(api_login))
}



#[derive(Debug, Deserialize)]
struct ApiLogin {
    username: String,
    pwd: String,
}


async fn api_login(payload: Json<ApiLogin>) -> Result<Json<Value>> {
    // Todo: implement the auth and db

    println!("->> {:<12} - api_login", "HANDLER");
    if payload.username != "demo" || payload.pwd != "lanmei" {
        return Err(Error::LoginFail);
    }

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