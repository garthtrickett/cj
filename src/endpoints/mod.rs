use crate::ichiran::run_docker_command;
use crate::ichiran_extraction::process_lines;
use color_eyre::eyre::Result;
use poem::error::InternalServerError;
use poem_openapi::{payload::PlainText, OpenApi};
use tokio::task::spawn_blocking;

pub struct Api;

#[OpenApi]
impl Api {
    // This endpoint processes a subtitle input and returns processed data.
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
                println!("Docker command ran successfully.");
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

                Ok(PlainText("Placeholder".to_string()))
            }
            Err(_) => Err(InternalServerError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to run docker command",
            ))),
        }
    }
}
