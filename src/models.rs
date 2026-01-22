use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Person {
    pub id: i64,
    pub name: String,
    pub age: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewPerson {
    pub name: String,
    pub age: i64,
}

#[derive(Serialize, Debug)]
pub struct PersonResponse {
    pub status: String,
    pub name: String,
    pub age: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonUpdate {
    pub name: Option<String>,
    pub age: Option<i64>,
}

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

impl Person {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().len() < 2 {
            return Err("The name is short, at least 2 characters are required".to_string());
        }
        if self.age < 0 || self.age > 120 {
            return Err("Age must be between 0 and 120".to_string());
        }
        Ok(())
    }
}

impl NewPerson {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().len() < 2 {
            return Err("The name is short, at least 2 characters are required".to_string());
        }
        if self.age < 0 || self.age > 120 {
            return Err("Age must be between 0 and 120".to_string());
        }
        Ok(())
    }
}

