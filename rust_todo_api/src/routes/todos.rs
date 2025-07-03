use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use crate::{models::todo::{Todo, TodoInput}, state::AppState};
use serde_json::json;

pub async fn list_todos(State(state): State<AppState>) -> impl IntoResponse {
    // 收集所有 Todo 为 Vec
    let todos: Vec<Todo> = state
        .todos
        .iter()
        .map(|entry| entry.value().clone())
        .collect();

    Json(todos)
}

pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<TodoInput>,
) -> impl IntoResponse {
    let id = rand::random::<u32>();
    let todo = Todo {
        id,
        title: payload.title,
        completed: false,
    };

    state.todos.insert(id, todo.clone());

    (StatusCode::CREATED, Json(todo))
}
