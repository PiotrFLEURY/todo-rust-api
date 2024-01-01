
mod presentation;
mod data;

use presentation::router::create_router;

#[tokio::main]
async fn main() {

    let router = create_router();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

