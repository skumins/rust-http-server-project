use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::{db::Db, models::*}; // Import from models 
use crate::error::AppError;

// -- Person Handlers --
pub async fn add_person(State(pool): State<Db>, Json(person): Json<NewPerson>,) -> Result <impl IntoResponse, AppError> {
    person.validate().map_err(AppError::InvalidData)?;
    
    let result= sqlx::query!("INSERT INTO persons (name, age) VALUES (?, ?)", 
        person.name,
        person.age)
        .execute(&pool)
        .await?;
    Ok((StatusCode::CREATED, format!("Created person with id: {}", result.last_insert_rowid())))
}

pub async fn get_all_persons(State(pool): State<Db>) -> Result<Json<Vec<Person>>, AppError> {
    let people = sqlx::query_as!(Person, "SELECT id, name, age FROM persons ORDER BY id")
        .fetch_all(&pool)
        .await?;
    Ok(Json(people))
}


pub async fn update_person(State(pool): State<Db>, Path(id): Path<i64>, Json(update): Json<PersonUpdate>,) -> Result <impl IntoResponse, AppError> {
    let mut query = String::from("UPDATE persons SET ");
    let mut params = Vec::new();

    if let Some(name) = &update.name {
        query.push_str("name = ?, ");
        params.push(name.clone());
    }

    if let Some(age) = update.age {
        query.push_str("age = ?, ");
        params.push(age.to_string());
    }

    query = query.trim_end_matches(", ").to_string();
    query.push_str(" WHERE id = ?");
    params.push(id.to_string());

    let result = sqlx::query(&query)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    
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
