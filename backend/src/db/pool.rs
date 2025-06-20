use dotenvy::dotenv;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn get_pool() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&url).await?;
    tracing::info!("Connected to the database!");
    Ok(pool)
}
