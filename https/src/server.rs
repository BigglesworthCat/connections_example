use serde::{Deserialize, Serialize};
use std::time::Duration;
use warp::Filter;

// Define the request and response structures
#[derive(Deserialize)]
struct ClientMessage {
    username: String,
    data: String,
}

#[derive(Serialize)]
struct ServerResponse {
    message: String,
}

// Asynchronous handler function
async fn handle_request(msg: ClientMessage) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Request from {}: {}", msg.username, msg.data);

    // Simulate a delay in processing the request
    tokio::time::sleep(Duration::from_secs(10)).await;

    println!("Request from {} handled", msg.username);

    // Send a response back to the client
    Ok(warp::reply::json(&ServerResponse {
        message: format!("Hello, {}! Server processed: {}", msg.username, msg.data),
    }))
}

#[tokio::main]
async fn main() {
    let send_route = warp::post()
        .and(warp::path("send"))
        .and(warp::body::json())
        .and_then(handle_request); // Call the async handler

    println!("Server running on http://127.0.0.1:12345");

    warp::serve(send_route).run(([127, 0, 0, 1], 12345)).await;
}