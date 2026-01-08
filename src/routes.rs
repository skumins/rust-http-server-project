use axum::{
    routing::{delete, get, post, put},
    Router,
};
use crate::{db::init_db, handlers}; // Import handlers and db init

pub fn create_router(db: crate::db::Db) -> Router {
    Router::new()
        .route("/person", post(handlers::add_person))
        .route("/persons", get(handlers::get_all_persons))
        .route("/person/{id}", put(handlers::update_person))
        .route("/person/{id}", delete(handlers::delete_person))
        .route("/calculate", post(handlers::add_number))
        .with_state(db)
}