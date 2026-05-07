use axum::{Json, extract::State};
use tracing::{info, error, debug};

use crate::{
    models::search::{SearchRequest, SearchResponse},
    services::search_service::SearchService,
};

#[axum::debug_handler]
pub async fn search_handler(
    State(service): State<SearchService>,
    Json(payload): Json<SearchRequest>,
) -> Json<SearchResponse> {

    info!("📥 Incoming /search request");

    debug!("Query: {}, k: {:?}", payload.query, payload.k);

    let k = payload.k.unwrap_or(5);

    let start = std::time::Instant::now();

    match service.search(payload.query.clone(), k).await {
        Ok(results) => {
            info!(
                "✅ Search successful | results: {} | took: {:?}",
                results.len(),
                start.elapsed()
            );

            Json(SearchResponse { results })
        }

        Err(e) => {
            error!("❌ Search failed: {}", e);

            Json(SearchResponse { results: vec![] })
        }
    }
}