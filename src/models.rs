use serde::{Deserialize, Serialize};

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