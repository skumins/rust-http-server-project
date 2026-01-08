use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::{db::Db, models::*}; // Import from models 

use crate::error::AppError;

// -- Basic Handlers --
pub async fn home() -> &'static str {
    "Welcome to the Home Page!"
}

pub async fn hello(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

// -- Person Handlers --
pub async fn add_person(State(pool): State<Db>, Json(person): Json<Person>,) -> Result <impl IntoResponse, AppError> {
    person.validate().map_err(AppError::InvalidData)?;
    
    sqlx::query!("INSERT INTO persons (name, age) VALUES (?, ?)", 
        person.name,
        person.age)
        .execute(&pool)
        .await?;
    Ok(StatusCode::CREATED)
}

pub async fn get_all_persons(State(pool): State<Db>) -> Result<Json<Vec<Person>>, AppError> {
    let people = sqlx::query_as!(Person, "SELECT id as \"id!\", name as \"name!\", age as \"age!\" FROM persons")
        .fetch_all(&pool)
        .await?;
    Ok(Json(people))
}


pub async fn update_person(State(pool): State<Db>, Path(id): Path<i64>, Json(update): Json<PersonUpdate>,) -> Result <impl IntoResponse, AppError> {
    sqlx::query!(
    "UPDATE persons SET name = COALESCE(?, name), age = COALESCE(?, age) WHERE id = ?",
        update.name,
        update.age,
        id
    )
    .execute(&pool)
    .await?;
    Ok(StatusCode::OK)
}

pub async fn delete_person(State(pool): State<Db>, Path(id): Path<i64>) -> Result <impl IntoResponse, AppError> {
    let result = sqlx::query!("DELETE FROM persons WHERE id = ?", id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}

// --- Math Handler ---
pub async fn add_number(Json(number): Json<Number>) -> Json<NumberResponse> {
    let sum = number.a + number.b;
    Json(NumberResponse {
        status: "received and calculated".to_string(),
        c: sum,
    })
}