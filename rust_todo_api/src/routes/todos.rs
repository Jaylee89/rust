use axum::{extract::{State, Json}, http::StatusCode, response::IntoResponse};
use crate::{models::todo::{Todo, TodoInput}, state::AppState};

pub async fn list_todos(State(state): State<AppState>) -> impl IntoResponse {
    let todos = state.todos.lock().unwrap();
    Json(todos.clone())
}

pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<TodoInput>,
) -> impl IntoResponse {
    let mut todos = state.todos.lock().unwrap();
    let id = todos.len() as u32 + 1;
    let todo = Todo {
        id,
        title: payload.title,
        completed: false,
    };
    todos.push(todo.clone());
    (StatusCode::CREATED, Json(todo))
}
