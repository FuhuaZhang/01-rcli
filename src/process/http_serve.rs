use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;

struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let state = HttpServeState { path: path.clone() };
    let serve_dir = ServeDir::new(path)
        .precompressed_br() //TODO
        .precompressed_deflate()
        .precompressed_gzip()
        .precompressed_zstd();
    let router = Router::new()
        .nest_service("/tower", serve_dir)
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("readign file {:?}", p);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("file not found: {:?}", p.display()),
        )
    } else {
        // TODO: test p is a directory, list all files/sub directories as <li> within <html> <body> <ul>
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to read file: {:?}", e),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, sync::Arc};

    use axum::{
        extract::{Path, State},
        http::StatusCode,
    };

    use crate::process::http_serve::{file_handler, HttpServeState};

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });

        let (status, content) = file_handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.contains("[package]"));
    }
}
