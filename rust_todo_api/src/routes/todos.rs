use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use crate::{models::todo::{Todo, TodoInput}, state::AppState};
use serde_json::json;

use axum::extract::Query;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pagination {
    page: Option<usize>,
    per_page: Option<usize>,
}

pub async fn list_todos(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> impl IntoResponse {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(20);
    let skip = (page - 1) * per_page;

    // Efficiently collect and paginate todos
    let todos = state
        .todos
        .iter()
        .skip(skip)
        .take(per_page)
        .map(|entry| entry.value().clone())
        .collect::<Vec<_>>();

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
