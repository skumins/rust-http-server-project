use axum::{
    routing::{delete, get, post, put},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use crate::handlers;

pub fn create_router(db: crate::db::Db) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/person", post(handlers::add_person))
        .route("/persons", get(handlers::get_all_persons))
        .route("/person/{id}", put(handlers::update_person))
        .route("/person/{id}", delete(handlers::delete_person))
        .route("/calculate", post(handlers::add_number))
        .layer(cors)
        .with_state(db)
}