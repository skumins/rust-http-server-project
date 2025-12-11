use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::{db::Db, models::*}; // Import from models 

// -- Basic Handlers --
pub async fn home() -> &'static str {
    "Welcome to the Home Page!"
}

pub async fn hello(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

// -- Person Handlers --
pub async fn add_person(State(db): State<Db>, Json(person): Json<Person>,) -> Json<PersonResponse> {
    let mut data = db.lock().await;
    data.push(person.clone());

    Json(PersonResponse {
        status: format!("Person '{}' added successfully", person.name),
        name: person.name,
        age: person.age,
    })
}

pub async fn get_all_persons(State(db): State<Db>) -> Json<Vec<Person>> {
    let people = db.lock().await;
    Json(people.clone())
}

pub async fn get_person(State(db): State<Db>, Path(name): Path<String>,) -> impl IntoResponse {
    let data = db.lock().await;
    for person in data.iter() {
        if person.name == name {
            return Json(person.clone()).into_response();
        }
    }
    (StatusCode::NOT_FOUND, "Person not found").into_response()
}

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

pub async fn delete_person(State(db): State<Db>, Path(name): Path<String>,) -> impl IntoResponse {
    let mut data = db.lock().await;
    if let Some(pos) = data.iter().position(|person| person.name == name) {
        data.remove(pos);
        return StatusCode::NO_CONTENT.into_response();
    }
    (StatusCode::NOT_FOUND, "Person not found").into_response()
}

// --- Math Handler ---
pub async fn add_number(Json(number): Json<Number>) -> Json<NumberResponse> {
    let sum = number.a + number.b;
    Json(NumberResponse {
        status: "received and calculated".to_string(),
        c: sum,
    })
}