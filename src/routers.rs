use axum::{
    Json, Router,
    routing::{get, post}, 
    extract::{Path, State},
};
use serde::{Deserialize, Serialize};

use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

#[derive(Serialize, Debug)]
pub struct PersonResponse {
    pub status: String,
    pub name: String,
    pub age: u8,
}

pub type Db = Arc<Mutex<Vec<Person>>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Number {
    pub a: u16,
    pub b: u16,
}

#[derive(Serialize, Debug)]
pub struct NumberResponse {
    pub status: String,
    pub c: u16,
}

// POST /person: add a new person
pub async fn add_person( State(db): State<Db>, Json(person): Json<Person>,) -> Json<PersonResponse> {
    let mut data = db.lock().await;
    data.push(person.clone());

    let response = PersonResponse {
        status: format!("Person '{}' added successfully", person.name),
        name: person.name,
        age: person.age,
    };
    Json(response)
}

// GET /persons: get all persons
pub async fn get_all_persons(State(db): State<Db>,) -> Json<Vec<Person>> {
    let people = db.lock().await;
    let people_clone = people.clone();
    Json(people_clone)
}


pub async fn add_number(Json(number): Json<Number>) -> Json<NumberResponse> {
    let sum = number.a + number.b;
    let response = NumberResponse {
        status: "received and calculated".to_string(),
        c: sum,
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
    let db: Db = Arc::new(Mutex::new(Vec::new()));
    Router::new()
        .route("/", get(home))
        .route("/hello/{name}", get(hello))
        .route("/person", post(add_person))
        .route("/calculate", post(add_number))
        .route("/persons", get(get_all_persons))
        .with_state(db)
}