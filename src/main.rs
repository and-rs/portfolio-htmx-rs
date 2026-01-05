use axum::{Router, routing::get};

async fn base_route() -> &'static str {
    "my final message, change the world"
}

async fn photos_route() -> &'static str {
    "this is the photos route"
}

async fn projects_route() -> &'static str {
    "this is the projects route"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(base_route))
        .route("/photos", get(photos_route))
        .route("/projects", get(projects_route));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
