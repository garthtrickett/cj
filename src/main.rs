use color_eyre::Report;
use poem::listener::TcpListener;
use poem::EndpointExt;
use poem::Route;
use poem::Server;
use poem_openapi::OpenApiService;
use sqlx::postgres::PgPool;
use std::env;
use tracing::debug;
use tracing_subscriber::FmtSubscriber;

mod endpoints;
mod models;

use crate::endpoints::todos::TodosApi;

#[tokio::main]
async fn main() -> Result<(), Report> {
    color_eyre::install()?;
    dotenv::dotenv().ok();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    debug!(feeling = "yay", "I'm gonna shave a yak also");

    let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap()).await?;
    let api_service =
        OpenApiService::new(TodosApi, "Todos", "1.0.0").server("[6](http://localhost:3000)");
    let ui = api_service.swagger_ui();
    let app = Route::new()
        .nest("/api", api_service)
        .nest("/", ui)
        .data(pool);

    if let Err(e) = Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
    {
        eprintln!("Server error: {}", e);
    }

    Ok(())
}
