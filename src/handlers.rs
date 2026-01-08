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
pub async fn add_person(State(pool): State<Db>, Json(person): Json<Person>,) -> impl IntoResponse {
    let result = sqlx::query!(
        "INSERT INTO persons (name, age) VALUES (?, ?)",
        person.name,
        person.age
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => (
            StatusCode::CREATED,
            Json(PersonResponse {
                status: format!("Person '{}' added successfully", person.name), 
                name: person.name,
                age: person.age
            })
        ).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn get_all_persons(State(pool): State<Db>) -> impl IntoResponse {
    let result = sqlx::query_as!(Person, "SELECT name as \"name!\", age as \"age!\" FROM persons")
        .fetch_all(&pool)
        .await;

    match result {
        Ok(people) => Json(people).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn get_person(State(pool): State<Db>, Path(name): Path<String>,) -> impl IntoResponse {
    let result = sqlx::query_as!(Person, "SELECT name as \"name!\", age as \"age!\" FROM persons WHERE name = ?", name)
        .fetch_optional(&pool)
        .await;

    match result {
        Ok(Some(person)) => Json(person).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Person not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn update_person(State(pool): State<Db>, Path(name): Path<String>, Json(update): Json<PersonUpdate>,) -> impl IntoResponse {
    let current = sqlx::query_as!(Person, "SELECT name as \"name!\", age as \"age!\" FROM persons WHERE name = ?", name)
        .fetch_optional(&pool)
        .await;

    if let Ok(Some(mut person)) = current {
        if let Some(new_name) = update.name {person.name = new_name;}
        if let Some(new_age) = update.age {person.age = new_age;}

        let update_result = sqlx::query!(
            "UPDATE persons SET name = ?, age = ? WHERE name = ?",
            person.name,
            person.age,
            name
        )
        .execute(&pool)
        .await;

        return match update_result {
            Ok(_) => Json(person).into_response(),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Update failed").into_response(),
        };
    }
    (StatusCode::NOT_FOUND, "Person not found").into_response()
}

pub async fn delete_person(State(pool): State<Db>, Path(name): Path<String>) -> impl IntoResponse {
    let result = sqlx::query!("DELETE FROM persons WHERE name = ?", name)
        .execute(&pool)
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => StatusCode::NO_CONTENT.into_response(),
        Ok(_) => (StatusCode::NOT_ACCEPTABLE, "Person not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

// --- Math Handler ---
pub async fn add_number(Json(number): Json<Number>) -> Json<NumberResponse> {
    let sum = number.a + number.b;
    Json(NumberResponse {
        status: "received and calculated".to_string(),
        c: sum,
    })
}