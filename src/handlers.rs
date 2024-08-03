use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use axum::extract::Path;
use diesel::prelude::*;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use crate::models::{NewTodo, Todo, UpdateTodo};
use crate::schema::todos;
use crate::schema::todos::id;

pub type DbPool = Arc<r2d2::Pool<ConnectionManager<PgConnection>>>;

pub async fn create_todo(
    State(db): State<DbPool>,
    Json(new_todo): Json<NewTodo>,
) -> (StatusCode,Json<Todo>) {
    let mut conn = db.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    let todo = diesel::insert_into(todos::table)
        .values(&new_todo)
        .get_result(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    (StatusCode::CREATED, Json(todo))
}

pub async fn get_todos(
    State(db): State<DbPool>,
) -> (StatusCode,Json<Vec<Todo>>) {
    let mut conn = db.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    let results = todos::table.load::<Todo>(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    (StatusCode::OK, Json(results))
}

pub async fn get_todo(
    Path(todo_id): Path<i32>,
    State(db): State<DbPool>,
) -> (StatusCode,Json<Todo>) {
    let mut conn = db.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    let result = todos::table.filter(id.eq(todo_id)).first::<Todo>(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    (StatusCode::OK, Json(result))
}

pub async fn update_todo(
    Path(todo_id): Path<i32>,
    State(db): State<DbPool>,
    Json(update_todo): Json<UpdateTodo>,
) -> (StatusCode,Json<Todo>) {
    let mut conn = db.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    let todo = diesel::update(todos::table.filter(id.eq(todo_id)))
        .set(&update_todo)
        .get_result(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    (StatusCode::OK, Json(todo))
}

pub async fn delete_todo(
    Path(todo_id): Path<i32>,
    State(db): State<DbPool>,
) -> StatusCode {
    let mut conn = db.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    let _ =diesel::delete(todos::table.filter(id.eq(todo_id)))
        .execute(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    StatusCode::NO_CONTENT
}