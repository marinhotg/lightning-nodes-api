mod handlers;
mod models;
mod repositories;
mod services;

use handlers::app;
use sqlx::PgPool;

pub async fn run() {
    let database_url = "postgresql://postgres:postgres@localhost:5432/lightning_nodes";
    let pool = PgPool::connect(database_url).await.unwrap();

    let app = app(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
