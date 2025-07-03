use dashmap::DashMap;
use std::sync::Arc;
use crate::models::todo::Todo;

#[derive(Clone)]
pub struct AppState {
    pub todos: Arc<DashMap<u32, Todo>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            todos: Arc::new(DashMap::new()),
        }
    }
}
