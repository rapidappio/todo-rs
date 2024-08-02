use crate::schema::*;
use diesel::prelude::*;
use diesel::sql_types::Bool;
use serde::{Deserialize, Serialize};

#[derive(Queryable,Serialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub content: String,
}

#[derive(Insertable,Deserialize)]
#[diesel(table_name = crate::schema::todos)]
pub struct NewTodo {
    pub title: String,
    pub content: String,
}