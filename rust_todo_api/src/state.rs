use std::sync::{Arc, Mutex};
use crate::models::todo::Todo;

#[derive(Clone)]
pub struct AppState {
    pub todos: Arc<Mutex<Vec<Todo>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            todos: Arc::new(Mutex::new(vec![])),
        }
    }
}
