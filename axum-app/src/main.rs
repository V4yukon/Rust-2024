use std::fs;

use axum::extract::Query;
use axum::response::{Html, IntoResponse};
// use std::net::SocketAddr;
// use axum::routing::Route;
use axum::Router;
use axum::routing::get;
use serde::Deserialize;
use tower_http::services::ServeDir;


#[allow(unused)]


#[tokio::main]

async fn main() {

    let static_file = ServeDir::new("static");
    // println!("Hello world!");
    let router_all = Router::new()
        .merge(router_path().await)
        .nest_service("/static", static_file);

    // let addr = SocketAddr::from(([127.0.0.1], 8080));
    let addr = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    // println!("Listening on {addr}");
    println!("->> LISTENING ON {:?}", addr.local_addr());
    // axum::serve(tcp_listener, make_service)
    axum::serve(addr, router_all)
        .await
        .unwrap();

}

async fn router_path() -> Router{
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
