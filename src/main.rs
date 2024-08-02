use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use axum::Router;
use axum::routing::post;
use diesel::{PgConnection, r2d2};
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;

mod models;
mod handlers;
mod schema;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let db_connection = Arc::new(pool);

    let app = Router::new()
        .route("/todos", post(handlers::create_todo))
        // .route("/todos", get(handlers::get_todos))
        // .route("/todos/:id", get(handlers::get_todo))
        // .route("/todos/:id", post(handlers::update_todo))
        // .route("/todos/:id", delete(handlers::delete_todo))
        .with_state(db_connection);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
