use crate::{
    model::{QueryOptions, Todo, UpdateTodoSchema},
    response::{GenericResponse, SingleTodoResponse, TodoData, TodoListResponse},
    WebResult, DB,
};
use chrono::prelude::*;
use uuid::Uuid;
use warp::{http::StatusCode, reply::json, reply::with_status, Reply};

pub async fn health_checker_handler() -> WebResult<impl Reply> {
    const MESSAGE: &str = "Simple CRUD API with Rust";
    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),    
    };
    Ok(json(response_json))
}

pub async fn todos_list_handler(opts: QueryOptions, db: DB) -> WebResult<impl Reply> {
    let todos = db.lock().await;

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let todos: Vec<Todo> = todos.clone().into_iter().skip(offset).take(limit).collect();

    let json_response = TodoListResponse {
        status: "success".to_string(),
        results: todos.len(),
        todos,
    };
    Ok(json(&json_response))
}