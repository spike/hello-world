
//     // run it
//     let listener = tokio::net::TcpListener::bind("[::]:8080")
//         .await
//         .unwrap();
//     println!("listening on {}", listener.local_addr().unwrap());
//     axum::serve(listener, app).await.unwrap();
// }

use axum::{
    extract::Query,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use hyper::server::conn::AddrIncoming;
use hyper::server::Server;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Build the application with a route
    let app = Router::new().route("/api/greet", get(greet_handler));

    // Bind to a generic address: [::]:8080 (IPv4 and IPv6)
    let listener = TcpListener::bind("[::]:8080")
        .await
        .expect("Failed to bind to address");
    let incoming = AddrIncoming::from_listener(listener).expect("Failed to create AddrIncoming");

    println!("Server running at http://[::]:8080");

    // Start the server
    Server::builder(incoming)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Query parameters structure
#[derive(Deserialize)]
struct GreetQuery {
    name: Option<String>,
}

// Response structure
#[derive(Serialize)]
struct GreetResponse {
    message: String,
}

// Handler for the /api/greet route
async fn greet_handler(Query(params): Query<GreetQuery>) -> impl IntoResponse {
    let name = params.name.unwrap_or_else(|| "World".to_string());
    let message = format!("Hello, {}!", name);

    Json(GreetResponse { message })
}
