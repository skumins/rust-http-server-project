use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::{db::Db, models::*}; // Import from models 
use crate::error::AppError;
use serde_json::json;

// -- Person Handlers --
pub async fn add_person(State(pool): State<Db>, Json(person): Json<NewPerson>,) -> Result <impl IntoResponse, AppError> {
    person.validate().map_err(AppError::InvalidData)?;
    
    let result= sqlx::query!("INSERT INTO persons (name, age) VALUES (?, ?)", 
        person.name,
        person.age)
        .execute(&pool)
        .await?;

        let response_body = json!({
            "status": "success",
            "id": result.last_insert_rowid()
        });
        Ok((StatusCode::CREATED, Json(response_body)))
}

pub async fn get_all_persons(State(pool): State<Db>) -> Result<Json<Vec<Person>>, AppError> {
    let people = sqlx::query_as!(Person, "SELECT id, name, age FROM persons ORDER BY id")
        .fetch_all(&pool)
        .await?;
    Ok(Json(people))
}


pub async fn update_person(State(pool): State<Db>, Path(id): Path<i64>, Json(update): Json<PersonUpdate>,) -> Result <impl IntoResponse, AppError> {
    // Check if person exists
    let _exists: (i64,) = sqlx::query_as("SELECT id FROM persons WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;

    // Build update query based on provided fields
    if update.name.is_none() && update.age.is_none() {
        return Ok((StatusCode::BAD_REQUEST, Json(json!({
            "status": "error",
            "message": "No fields to update"
        }))));
    }

    if let (Some(name), Some(age)) = (&update.name, update.age) {
        sqlx::query("UPDATE persons SET name = ?, age = ? WHERE id = ?")
            .bind(name)
            .bind(age)
            .bind(id)
            .execute(&pool)
            .await?;
    } else if let Some(name) = &update.name {
        sqlx::query("UPDATE persons SET name = ? WHERE id = ?")
            .bind(name)
            .bind(id)
            .execute(&pool)
            .await?;
    } else if let Some(age) = update.age {
        sqlx::query("UPDATE persons SET age = ? WHERE id = ?")
            .bind(age)
            .bind(id)
            .execute(&pool)
            .await?;
    }
    
    Ok((StatusCode::OK, Json(json!({
        "status": "success",
        "message": "Person updated successfully"
    }))))
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
