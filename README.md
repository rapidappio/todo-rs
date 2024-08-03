##  Building a Todo API with Rust - A Step-by-Step Guide Using Axum and Diesel
This is the project demonstrating how to build a Todo REST API by using Rust, Axum, Diesel and PostgreSQL

### Requirements
- PostgreSQL database

### How to run
1. `export DATABASE_URL=...` The format should be `postgresql://host:port/db_name?sslmode=require&application_name=todo-rs`. You can easily create PostgreSQL database for free [here](https://rapidapp.io)
2. `cargo run`