use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

/// So we don't have to tackle how different database work, we'll just use
/// a simple in-memory DB, a vector synchronized by a mutex.
pub type Db = Arc<Mutex<Vec<Function>>>;

pub fn blank_db() -> Db {
    Arc::new(Mutex::new(Vec::new()))
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Function {
    pub id: u64,
    pub code: String,
    pub language: String,
    pub completed: bool,
}

// The query parameters for list_functions.
#[derive(Debug, Deserialize)]
pub struct ListOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}