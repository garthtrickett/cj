use color_eyre::eyre::Result;
use color_eyre::Report;
use poem::error::InternalServerError;
use poem::listener::TcpListener;
use poem::web::Data;
use poem::EndpointExt;
use poem::Error;
use poem::Route;
use poem::Server;
use poem_openapi::payload::Json;
use poem_openapi::Object;
use poem_openapi::OpenApiService;
use poem_openapi::{payload::PlainText, OpenApi};
use sqlx::postgres::PgPool;
use std::env;
use tracing::debug;
use tracing_subscriber::FmtSubscriber;

#[derive(Object)]
struct Todo {
    id: i32,
    description: String,
    done: bool,
}

type TodoResponse = Result<Json<Vec<Todo>>, Error>;

struct TodosApi;

#[OpenApi]
impl TodosApi {
    #[oai(path = "/todos", method = "post")]
    async fn create(
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
    async fn get_all(&self, pool: Data<&PgPool>) -> TodoResponse {
        let todos = sqlx::query_as!(Todo, "SELECT * FROM todos")
            .fetch_all(pool.0)
            .await
            .unwrap();

        Ok(Json(todos))
    }
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    color_eyre::install()?;
    dotenv::dotenv().ok();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    debug!(feeling = "yay", "I'm gonna shave a yak.");

    let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap()).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
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
