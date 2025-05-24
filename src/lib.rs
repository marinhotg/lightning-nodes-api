mod handlers;
mod services;

use handlers::app;

pub async fn run() {
    let app = app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
