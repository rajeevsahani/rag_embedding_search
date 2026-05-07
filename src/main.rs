mod handlers;
mod services;
mod repositories;
mod clients;
mod models;
mod errors;
mod config;

use axum::{Router, routing::post};
use handlers::search_handler::search_handler;
use services::search_service::SearchService;
use repositories::search_repository::SearchRepository;
use clients::embedding_client::EmbeddingClient;
use services::cache::EmbeddingCache;

use config::log::init_logger;

use tracing::{info, error, debug};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // 🔥 Initialize logger
    init_logger();

    info!("🚀 Starting Rust RAG server");

    // 🔹 Load DB config
    let db_url = "host=localhost user=postgres dbname=rag_db";
    debug!("Loaded DB config");

    // 🔹 Init DB pool
    let pool = config::db::create_pool(db_url).await;
    info!("✅ Database pool initialized");

    let repo = SearchRepository { pool };

    // 🔹 Init HTTP client
    let http = reqwest::Client::new();
    info!("🌐 HTTP client initialized");

    let embedding_client = EmbeddingClient { http };

    // 🔹 Init cache
    let cache = EmbeddingCache::new();
    info!("🧠 Cache initialized");

    // 🔹 Build service
    let service = SearchService {
        repo,
        embedding_client,
        cache,
    };

    info!("⚙️ Service layer constructed");

    // 🔹 Build router
    let app = Router::new()
        .route("/search", post(search_handler))
        .with_state(service);

    info!("🛣️ Routes registered");

    // 🔹 Bind server
    let listener = match TcpListener::bind("0.0.0.0:3000").await {
        Ok(l) => {
            info!("🌍 Server listening on http://127.0.0.1:3000");
            l
        }
        Err(e) => {
            error!("❌ Failed to bind server: {}", e);
            return;
        }
    };

    // 🔹 Serve
    if let Err(e) = axum::serve(listener, app).await {
        error!("❌ Server crashed: {}", e);
    }
}