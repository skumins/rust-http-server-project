use axum::{
    routing::{get, post, MethodRouter},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use crate::handlers;

pub fn create_router(db: crate::db::Db) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let update_delete_route = MethodRouter::new()
        .put(handlers::update_person)
        .delete(handlers::delete_person);

    Router::new()
        .route("/person/update/:id", update_delete_route.clone())
        .route("/person/delete/:id", update_delete_route)
        .route("/person", post(handlers::add_person))
        .route("/persons", get(handlers::get_all_persons))
        .route("/calculate", post(handlers::add_number))
        .layer(cors)
        .with_state(db)
}