use crate::endpoints::Api;
use color_eyre::Report;
use deadpool_postgres::{Config, Runtime};
use poem::listener::TcpListener;
use poem::EndpointExt;
use poem::Route;
use poem::Server;
use poem_openapi::OpenApiService;
use tokio_postgres::NoTls;
use tracing::debug;
use tracing_subscriber::FmtSubscriber;
mod cornucopia;
mod endpoints;
mod ichiran;

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
    // Connection pool configuration
    // This has nothing to do with cornucopia, please look at
    // `tokio_postgres` and `deadpool_postgres` for details
    let mut cfg = Config::new();
    cfg.user = Some(String::from("postgres"));
    cfg.password = Some(String::from("password"));
    cfg.host = Some(String::from("127.0.0.1"));
    cfg.port = Some(5432);
    cfg.dbname = Some(String::from("cj"));
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

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
