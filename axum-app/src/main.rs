#[allow(unused)]
use std::fs;
use axum::response::Response;
use axum::extract::Query;
use axum::response::{Html, IntoResponse};
// use std::net::SocketAddr;
// use axum::routing::Route;
use axum::{Router, middleware};
use axum::routing::get;
use serde::Deserialize;
use tower_http::services::ServeDir;
use tower_cookies::CookieManagerLayer;

mod error;
mod web;
mod rjwt;
mod model;
// use web::*;
use crate::error::{Error, Result};
use crate::model::ModelController;

#[allow(unused)]


#[tokio::main]

async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let static_file = ServeDir::new("static");

    let router_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));
    // println!("Hello world!");
    let router_all = Router::new()
        .merge(router_path())
        .merge(web::routes_login::routes())
        .nest("/api", router_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .nest_service("/static", static_file);

    // let addr = SocketAddr::from(([127.0.0.1], 8080));
    let addr = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    // println!("Listening on {addr}");
    println!("->> LISTENING ON {:?}", addr.local_addr());
    // axum::serve(tcp_listener, make_service)
    axum::serve(addr, router_all)
        .await
        .unwrap();

    Ok(())

}

// Res-mapper
async fn main_response_mapper(res: Response) -> Response{
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res

}

 fn router_path() -> Router{
    Router::new()
        .route("/", get(root))
        .route("/hello", get(hello_handler))
        
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn hello_handler(Query(params): Query<HelloParams>) ->  impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!!!");

    Html(format!("Hello <strong>{name}</strong>"))
}

async fn root() -> Html<String> {
    let text_content = fs::read_to_string("static/index.html").expect("Failed to read html");
    Html(text_content)
}
