use color_eyre::eyre::Result;
use poem::error::InternalServerError;
use poem::web::Data;
use poem::Error;
use poem_openapi::payload::Json;
use poem_openapi::{payload::PlainText, OpenApi};
use sqlx::postgres::PgPool;

use crate::models::todo::Todo;

pub type TodoResponse = Result<Json<Vec<Todo>>, Error>;

pub struct TodosApi;

#[OpenApi]
impl TodosApi {
    #[oai(path = "/todos", method = "post")]
    pub async fn create(
        &self,
        pool: Data<&PgPool>,
        description: PlainText<String>,
    ) -> Result<Json<i32>, Error> {
        let row = sqlx::query!(
            r#"INSERT INTO todos (description, done) VALUES ($1, $2) RETURNING id"#,
            description.0,
            false // Default value for `done`
        )
        .fetch_one(pool.0)
        .await
        .map_err(InternalServerError)?;

        let id = row.id;

        Ok(Json(id))
    }

    #[oai(path = "/todos", method = "get")]
    pub async fn get_all(&self, pool: Data<&PgPool>) -> TodoResponse {
        let todos = sqlx::query_as!(Todo, "SELECT * FROM todos")
            .fetch_all(pool.0)
            .await
            .unwrap();

        Ok(Json(todos))
    }
}
