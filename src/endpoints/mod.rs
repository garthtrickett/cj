use crate::cornucopia::queries::module_1::example_query;
use crate::ichiran::extract_first_pos_tags;
use crate::ichiran::find_grammar_rules;
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
        println!("{:?}", output);
        match output {
            Ok(output_str) => {
                let lines: Vec<&str> = output_str.lines().collect();

                let pos_tags = extract_first_pos_tags(lines.clone()).map_err(|_| {
                    InternalServerError(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to extract grammar",
                    ))
                })?;

                let sentence_with_bracket_furigana = ichiran_output_to_bracket_furigana(lines)
                    .map_err(|_| {
                        InternalServerError(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Failed to process output from docker command",
                        ))
                    })?;

                // Define your rules here
                let rules: Vec<Vec<String>> = vec![
                    vec!["n,pn,adj-i,adj-na".to_string(), "です".to_string()],
                    vec!["n,pn,adj-na".to_string(), "だ".to_string()],
                    vec!["n,pn".to_string(), "も".to_string()],
                    vec!["は".to_string()],
                    vec!["これ".to_string()],
                    vec!["それ".to_string()],
                    vec!["あれ".to_string()],
                    vec!["n,pn".to_string(), "の".to_string(), "n,pn".to_string()],
                    vec!["いい".to_string()],
                    vec!["よくない".to_string()],
                    vec!["よかった".to_string()],
                    vec!["よくなかった".to_string()],
                    vec!["いいです".to_string()],
                    vec!["よくないです".to_string()],
                    vec!["よかったです".to_string()],
                    vec!["よくなかったです".to_string()],
                    vec!["adj-na".to_string(), "な".to_string()],
                    vec!["か".to_string()],
                ];

                let grammar_rules =
                    find_grammar_rules(sentence_with_bracket_furigana, pos_tags, rules).map_err(
                        |_| {
                            InternalServerError(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                "Failed to find grammar rules",
                            ))
                        },
                    )?;

                println!("Grammar rules: {:?}", grammar_rules); // Print the grammar rules

                Ok(PlainText("Placeholder".to_string()))
            }
            Err(_) => Err(InternalServerError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to run docker command",
            ))),
        }
    }
}
