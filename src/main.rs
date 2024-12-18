mod db;
mod handlers;
mod models;
mod routes;

use axum::Server;
// Ensure this import is correct
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // Build the application with routes
    let app = routes::app_router()
        // Add middleware layers
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    // Define the address to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    // Start the server
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
