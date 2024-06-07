use crate::endpoints::Api;
use color_eyre::Report;
use poem::listener::TcpListener;
use poem::Route;
use poem::Server;
use poem_openapi::OpenApiService;
use tracing::debug;
use tracing_subscriber::FmtSubscriber;
mod endpoints;
mod ichiran;
mod ichiran_extraction;

// Add more schema files and queries, rebuild the crate,
// and observe how your cornucopia modules are regenerated!
#[tokio::main]
async fn main() -> Result<(), Report> {
    color_eyre::install()?;
    dotenv::dotenv().ok();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();

    debug!("test");

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let api_service = OpenApiService::new(Api, "Api", "1.0.0").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service).nest("/", ui);

    if let Err(e) = Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
    {
        eprintln!("Server error: {}", e);
    }

    Ok(())
}
