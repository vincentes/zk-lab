use axum::{
    routing::{get, post},
    Router,
    extract::Json,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tower_http::cors::{CorsLayer, Any};

#[derive(Deserialize)]
struct RegisterRequest {
    commitment: String,
}

#[derive(Serialize)]
struct RegisterResponse {
    commitment: String,
}

impl IntoResponse for RegisterResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

async fn register(Json(payload): Json<RegisterRequest>) -> RegisterResponse {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("commitments")
        .await
        .unwrap();
    
    file.write_all(format!("{}\n", payload.commitment).as_bytes())
        .await
        .unwrap();

    RegisterResponse {
        commitment: payload.commitment,
    }
}

#[derive(Serialize)]
struct RegistryResponse {
    commitments: Vec<String>,
}

impl IntoResponse for RegistryResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

async fn registry() -> RegistryResponse {
    let mut file = OpenOptions::new()
        .read(true)
        .open("commitments")
        .await
        .unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).await.unwrap();

    RegistryResponse {
        commitments: contents.lines().map(|s| s.to_string()).collect(),
    }
}

async fn path(Json(user_id): Json<serde_json::Value>) -> String {
    format!("Hello, World! {}", user_id)
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/registry", post(register))
        .route("/registry", get(registry))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

