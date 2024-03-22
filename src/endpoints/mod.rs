use crate::cornucopia::queries::module_1::example_query;
use crate::ichiran::run_docker_command;
use crate::ichiran_extraction::process_lines;
use crate::sentence_processing::load_rules;
use crate::sentence_processing::match_rules;
// use color_eyre::config::PanicHook;
use color_eyre::eyre::Result;
use deadpool_postgres::Pool;
use poem::error::InternalServerError;
use poem::web::Data;
use poem::web::Query;
use poem::Error;
use poem_openapi::payload::Json;
use poem_openapi::{payload::PlainText, OpenApi};
use tokio::task::spawn_blocking;

pub type Response = Result<Json<Vec<String>>, Error>;

pub struct Api;

#[OpenApi]
impl Api {
    // Voldemort
    // This is the echo endpoint that returns a greeting message.
    #[oai(path = "/hello", method = "get")]
    async fn echo(&self, name: Query<Option<String>>) -> PlainText<String> {
        println!("Received a request for the echo endpoint.");
        match name.0 {
            Some(name) => PlainText(name),
            None => PlainText("hi!".to_string()),
        }
    }
    // This endpoint creates a new todo item and returns its ID.
    #[oai(path = "/todos", method = "post")]
    pub async fn create(
        &self,
        pool: Data<&Pool>,
        _description: PlainText<String>,
    ) -> Result<Json<i32>, Error> {
        let client = pool.get().await.map_err(InternalServerError)?;
        println!("Obtained database client for creating a todo.");

        let _result = example_query()
            .bind(&client)
            .all()
            .await
            .map_err(InternalServerError)?;

        let id = 3;
        println!("Created a new todo with ID: {}", id);

        Ok(Json(id))
    }

    // This endpoint retrieves all todo items.
    #[oai(path = "/todos", method = "get")]
    pub async fn get_all(&self, pool: Data<&Pool>) -> Response {
        println!("Received a request to get all todos.");
        let client = pool.get().await.map_err(InternalServerError)?;
        println!("Obtained database client for retrieving todos.");

        let result = example_query()
            .bind(&client)
            .all()
            .await
            .map_err(InternalServerError)?;

        Ok(Json(result))
    }

    // This endpoint processes a subtitle input and returns processed data.
    #[oai(path = "/subtitle", method = "post")]

    pub async fn subtitle(
        &self,
        input: PlainText<String>,
    ) -> Result<PlainText<String>, poem::Error> {
        let output = spawn_blocking(move || run_docker_command(&input))
            .await
            .map_err(|_| {
                println!("Failed to run docker command.");
                InternalServerError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to run docker command",
                ))
            })?;
        match output {
            Ok(output_str) => {
                println!("Docker command ran successfully.");
                // let output_str_hard = "食べない".to_string();
                let lines: Vec<&str> = output_str.lines().collect();

                let result = process_lines(lines).map_err(|_| {
                    InternalServerError(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to process output from docker command",
                    ))
                })?;

                for line in &result {
                    println!("{:?}\n", line);
                }

                let rules = load_rules();
                println!("Loaded rules for sentence processing.");

                let matched_rules = match_rules(result, rules);
                println!("Matched rules: {:?}", matched_rules);
                println!("{:?}", matched_rules);
                // Define your rules here
                // let rules: Vec<Vec<String>> = vec![
                //     ...
                //     // 1. These look for a part of speech (one of the ones seperated by comams) followed by a word
                //     vec!["n,pn,adj-i,adj-na".to_string(), "です".to_string()],
                //     vec!["n,pn,adj-na".to_string(), "だ".to_string()],
                //     vec!["n,pn".to_string(), "も".to_string()],
                //     vec!["adj-na".to_string(), "な".to_string()],
                //     // 2.These rules look for a word
                // vec!["は".to_string()],
                // vec!["これ".to_string()],
                // vec!["それ".to_string()],
                // vec!["あれ".to_string()],
                // vec!["いい".to_string()],
                // vec!["よくない".to_string()],
                // vec!["よかった".to_string()],
                // vec!["よくなかった".to_string()],
                // vec!["いいです".to_string()],
                // vec!["よくないです".to_string()],
                // vec!["よかったです".to_string()],
                // vec!["よくなかったです".to_string()],
                // vec!["か".to_string()],
                //     // 3. This one looks for a part of speech followed by the word の followed by another part of speech (same commas as to say it can be any of these)
                //     vec!["n,pn".to_string(), "の".to_string(), "n,pn".to_string()],
                //     // 4. these ones just look for a part of speech by itself
                // vec!["v1".to_string()],  // ichidan verbs
                // vec!["v5r".to_string()], // godan verbs
                // ];

                // let grammar_rules =
                //     find_grammar_rules(sentence_array, pos_tags, rules).map_err(|_| {
                //         InternalServerError(std::io::Error::new(
                //             std::io::ErrorKind::Other,
                //             "Failed to find grammar rules",
                //         ))
                //     })?;

                // println!("Grammar rules: {:?}", grammar_rules); // Print the grammar rules

                Ok(PlainText("Placeholder".to_string()))
            }
            Err(_) => Err(InternalServerError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to run docker command",
            ))),
        }
    }
}
