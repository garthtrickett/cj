use crate::cornucopia::queries::module_1::example_query;
use crate::ichiran::ichiran_output_to_bracket_furigana;
use crate::ichiran::run_docker_command;
use color_eyre::eyre::Result;
use deadpool_postgres::Pool;
use poem::error::InternalServerError;
use poem::web::Data;
use poem::Error;
use poem_openapi::payload::Json;
use poem_openapi::{payload::PlainText, OpenApi};
use tokio::task::spawn_blocking;

pub type Response = Result<Json<Vec<String>>, Error>;

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/todos", method = "post")]
    pub async fn create(
        &self,
        pool: Data<&Pool>,
        description: PlainText<String>,
    ) -> Result<Json<i32>, Error> {
        println!("{:?}", description);

        let client = pool.get().await.map_err(InternalServerError)?;

        let result = example_query()
            .bind(&client)
            .all()
            .await
            .map_err(InternalServerError)?;
        println!("{:?}", result.len());

        let id = 3;

        Ok(Json(id))
    }

    #[oai(path = "/todos", method = "get")]
    pub async fn get_all(&self, pool: Data<&Pool>) -> Response {
        let client = pool.get().await.map_err(InternalServerError)?;

        let result = example_query()
            .bind(&client)
            .all()
            .await
            .map_err(InternalServerError)?;
        println!("{:?}", result);

        Ok(Json(result))
    }

    #[oai(path = "/subtitle", method = "post")]
    pub async fn subtitle(
        &self,
        input: PlainText<String>,
    ) -> Result<PlainText<String>, poem::Error> {
        let output = spawn_blocking(move || run_docker_command(&input))
            .await
            .map_err(|_| {
                InternalServerError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to run docker command",
                ))
            })?;
        match output {
            Ok(output_str) => {
                match ichiran_output_to_bracket_furigana(&output_str) {
                    Ok(furigana_output) => {
                        let furigana_output_str = furigana_output.join(", "); // Convert Vec<String> to String
                        Ok(PlainText(furigana_output_str))
                    }
                    Err(_) => Err(InternalServerError(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to process output from docker command",
                    ))),
                }
            }
            Err(_) => Err(InternalServerError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to run docker command",
            ))),
        }
    }
}
