use axum::{
    routing::{get, post},
    Router,
    response::IntoResponse,
    extract::Path,
    http::StatusCode,
};
use std::{net::SocketAddr, path::PathBuf};
use tower_http::cors::{CorsLayer, Any};
use walkdir::WalkDir;
use std::fs;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(|| async { "Hello from Rust backend!" }))
        .route("/files", get(list_files))
        .route("/file/:path", get(get_file))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn list_files() -> impl IntoResponse {
    let mut files = Vec::new();
    let workspace_dir = PathBuf::from("/workspace");

    for entry in WalkDir::new(workspace_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        if let Ok(path) = entry.path().strip_prefix("/workspace") {
            if let Some(path_str) = path.to_str() {
                files.push(path_str.to_string());
            }
        }
    }

    (StatusCode::OK, axum::Json(files))
}

async fn get_file(Path(path): Path<String>) -> impl IntoResponse {
    let file_path = PathBuf::from("/workspace").join(path);
    
    match fs::read_to_string(&file_path) {
        Ok(content) => (StatusCode::OK, content).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "File not found").into_response(),
    }
}
