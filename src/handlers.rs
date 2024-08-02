use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use diesel::prelude::*;

use crate::models::{NewTodo, Todo};
use crate::schema::todos;

type DatabaseConnection = Arc<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>;

pub async fn create_todo(
    State(db): State<DatabaseConnection>,
    Json(new_todo): Json<NewTodo>,
) -> (StatusCode,Json<Todo>) {
    let mut conn = db.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    let todo = diesel::insert_into(todos::table)
        .values(&new_todo)
        .get_result(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    (StatusCode::CREATED, Json(todo))
}