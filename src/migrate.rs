// migrate.rs
use sqlx::postgres::PgPool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap()).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("Migration completed successfully.");
    Ok(())
}
