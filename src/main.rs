use std::net::SocketAddr;
use tokio::net::TcpListener;

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
    let app = routes::create_router(db);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}