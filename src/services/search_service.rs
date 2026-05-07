use pgvector::Vector;

use crate::{
    repositories::search_repository::SearchRepository,
    clients::embedding_client::EmbeddingClient,
    services::cache::EmbeddingCache,
    models::search::SearchResult,
    errors::app_error::AppError,
};

use tracing::{info, error, debug, info_span};

#[derive(Clone)]
pub struct SearchService {
    pub repo: SearchRepository,
    pub embedding_client: EmbeddingClient,
    pub cache: EmbeddingCache,
}

impl SearchService {
    pub async fn search(&self, query: String, k: i64) -> Result<Vec<SearchResult>, AppError> {

        // 🔥 Span groups all logs for this request
        let span = info_span!("search_service", k = k, query = %query);
        let _enter = span.enter();

        info!("🚀 Starting search service");

        let total_start = std::time::Instant::now();

        // 🔹 Step 1: Cache check
        let embedding = if let Some(cached) = self.cache.get(&query).await {
            info!("⚡ Using cached embedding");
            cached
        } else {
            info!("❌ Cache miss → fetching embedding");

            let embed_start = std::time::Instant::now();

            let emb = match self.embedding_client.get_embedding(&query).await {
                Ok(e) => {
                    info!(
                        "✅ Embedding fetched | dim={} | took: {:?}",
                        e.len(),
                        embed_start.elapsed()
                    );
                    e
                }
                Err(e) => {
                    error!("❌ Embedding failed: {}", e);
                    return Err(AppError::EmbeddingError(e));
                }
            };

            // store in cache
            self.cache.set(query.clone(), emb.clone()).await;
            debug!("💾 Stored embedding in cache");

            emb
        };

        // 🔹 Step 2: Vector conversion
        let vector = Vector::from(embedding);
        debug!("🔢 Converted embedding to vector");

        // 🔹 Step 3: DB search
        let db_start = std::time::Instant::now();

        let rows = match self.repo.search(vector, k).await {
            Ok(r) => {
                info!(
                    "✅ DB search completed | rows={} | took: {:?}",
                    r.len(),
                    db_start.elapsed()
                );
                r
            }
            Err(e) => {
                error!("❌ DB search failed: {}", e);
                return Err(e); // ✅ already AppError
            }
        };

        // 🔹 Step 4: Map results
        let results: Vec<SearchResult> = rows
            .into_iter()
            .map(|(content, score)| SearchResult { content, score })
            .collect();

        info!(
            "🎯 Search completed | results={} | total_time={:?}",
            results.len(),
            total_start.elapsed()
        );

        Ok(results)
    }
}