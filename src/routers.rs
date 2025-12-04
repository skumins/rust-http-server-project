use axum::{routing::get, Router, extract::Path};

pub async fn home() -> &'static str {
    "Welcome to the Home Page!"
}

pub async fn hello(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/hello/{name}", get(hello))
}