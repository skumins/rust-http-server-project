use axum::{
    routing::{delete, get, post, put},
    Router,
};
use crate::{db::init_db, handlers}; // Import handlers and db init

pub fn create_router() -> Router {
    let db = init_db();

    Router::new()
        .route("/", get(handlers::home))
        .route("/hello/{name}", get(handlers::hello))
        .route("/person", post(handlers::add_person))
        .route("/persons", get(handlers::get_all_persons))
        .route("/person/{name}", get(handlers::get_person))
        .route("/person/{name}", put(handlers::update_person))
        .route("/person/{name}", delete(handlers::delete_person))
        .route("/calculate", post(handlers::add_number))
        .with_state(db)
}