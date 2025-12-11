use axum::{
    Json, Router,
    routing::{get, post, put, delete}, 
    extract::{Path, State},
    response::{IntoResponse},
    http::{StatusCode},
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

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonUpdate {
    pub name: Option<String>,
    pub age: Option<u8>,
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

// Read One (GET/person/{name})
pub async fn get_person(State(db): State<Db>, Path(name): Path<String>,) -> impl IntoResponse {
    let data = db.lock().await;
    for person in data.iter() {
        if person.name == name {
            return Json(person.clone()).into_response();
        }
    }
    (StatusCode::NOT_FOUND, "Person not found").into_response()
}

// Update (PUT/person/{name})
pub async fn update_person(State(db): State<Db>, Path(name): Path<String>, Json(update): Json<PersonUpdate>,) -> impl IntoResponse {
    let mut data = db.lock().await;
    for person in data.iter_mut() {
        if person.name == name {
            if let Some(new_name) = update.name {
                person.name = new_name;
            }
            if let Some(new_age) = update.age {
                person.age = new_age;
            }
            return Json(person.clone()).into_response();
        }
    }
    (StatusCode::NOT_FOUND, "Person not found").into_response()
}

// DELETE
pub async fn delete_person(State(db): State<Db>, Path(name): Path<String>,) -> impl IntoResponse {
    let mut data = db.lock().await;
    if let Some(pos) = data.iter().position(|person| person.name == name) {
        data.remove(pos);
        return StatusCode::NO_CONTENT.into_response();
    }
    (StatusCode::NOT_FOUND, "Person not found").into_response()
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
    .route("/person/{name}", get(get_person)) 
    .route("/person/{name}", put(update_person))   
    .route("/person/{name}", delete(delete_person)) 
    .with_state(db)
}