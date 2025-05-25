mod handlers;
mod models;
mod repositories;
mod services;

use handlers::app;
use sqlx::PgPool;

pub async fn run() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.unwrap();

    tokio::spawn(services::scheduler::start_scheduler(pool.clone()));

    let app = app(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
