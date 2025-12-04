use axum::{Json, routing::{get, post}, Router, extract::Path};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Person {
    name: String,
    age: u8,
}

#[derive(Serialize)]
pub struct PersonResponse {
    status: String,
    name: String,
    age: u8,
}

pub async fn add_person(Json(person): Json<Person>) -> Json<PersonResponse> {
    let response = PersonResponse {
        status: "received".to_string(),
        name: person.name,
        age: person.age,
    };
    Json(response)
}

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
        .route("/person", post(add_person))
}