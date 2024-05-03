use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn};

struct HttpServeState {
    path: PathBuf,
}
pub async fn process_http_server(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", path, addr);
    let state = HttpServeState { path: path.clone() };
    // axum router
    let router = Router::new()
        .nest_service("/tower", ServeDir::new(path))
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> Result<Html<String>, (StatusCode, String)> {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    if !p.exists() {
        Err((
            StatusCode::NOT_FOUND,
            format!("File {} not found", p.display()),
        ))
    } else {
        // TODO: test p is a directory
        // if it is a directory, list all files/subdirectories
        // as <li><a href="/path/to/file">file name</a></li>
        // <html><body><ul>...</ul></body></html>
        if p.is_dir() {
            let mut content = String::new();
            content.push_str("<html><body><ul>");

            let mut content = String::new();
            content.push_str("<html><body><ul>");

            let mut dir_entries = tokio::fs::read_dir(p).await.unwrap();
            while let Some(entry) = dir_entries.next_entry().await.unwrap() {
                let entry_name = entry.file_name();
                let entry_name = entry_name.to_string_lossy();
                content.push_str(&format!(
                    "<li><a href=\"{}\">{}</a></li>",
                    entry_name, entry_name
                ));
            }

            content.push_str("</ul></body></html>");
            return Ok(Html(content));
        }

        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                Ok(Html(content))
            }
            Err(e) => {
                warn!("Error reading file: {:?}", e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error reading file".to_string(),
                ))
            }
        }
    }
}
