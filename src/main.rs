use std::net::SocketAddr;
use tokio::net::TcpListener;
use axum::Router;
use tower_http::services::ServeDir;

// declare modules
mod db;
mod handlers;
mod models;
mod routes;
mod error;


#[tokio::main]
async fn main() {
    let db = db::init_db().await;
    // Creating router through function in routes.rs
    let api_router = routes::create_router(db);

    let static_router = Router::new()
    .nest_service("/", ServeDir::new("frontent"));

    let app = Router::new()
    .nest("/api", api_router).merge(static_router);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}