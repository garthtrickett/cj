use crate::cornucopia::queries::module_1::example_query;
use color_eyre::eyre::Result;
use deadpool_postgres::Pool;
use poem::error::InternalServerError;
use poem::web::Data;
use poem::Error;
use poem_openapi::payload::Json;
use poem_openapi::{payload::PlainText, OpenApi};

pub type TodoResponse = Result<Json<Vec<String>>, Error>;

pub struct TodosApi;

#[OpenApi]
impl TodosApi {
    #[oai(path = "/todos", method = "post")]
    pub async fn create(
        &self,
        pool: Data<&Pool>,
        description: PlainText<String>,
    ) -> Result<Json<i32>, Error> {
        // let row = sqlx::query!(
        //     r#"INSERT INTO todos (description, done) VALUES ($1, $2) RETURNING id"#,
        //     description.0,
        //     false // Default value for `done`
        // )
        // .fetch_one(pool.0)
        // .await
        // .map_err(InternalServerError)?;

        println!("{:?}", description);

        let client = pool.get().await.map_err(InternalServerError)?;

        let result = example_query()
            .bind(&client)
            .all()
            .await
            .map_err(InternalServerError)?;
        println!("{:?}", result);

        let id = 3;

        Ok(Json(id))
    }

    #[oai(path = "/todos", method = "get")]
    pub async fn get_all(&self, pool: Data<&Pool>) -> TodoResponse {
        let client = pool.get().await.map_err(InternalServerError)?;

        let result = example_query()
            .bind(&client)
            .all()
            .await
            .map_err(InternalServerError)?;
        println!("{:?}", result);

        Ok(Json(result))
    }
}
