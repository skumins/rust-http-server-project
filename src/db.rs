use std::sync::Arc;
use tokio::sync::Mutex;
use crate::models::Person; // Import models

pub type Db = Arc<Mutex<Vec<Person>>>;

pub fn init_db() -> Db {
    Arc::new(Mutex::new(Vec::new()))
}