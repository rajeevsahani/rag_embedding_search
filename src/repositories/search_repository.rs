use crate::{
    config::db::DbPool,
    errors::app_error::AppError,
};
use pgvector::Vector;
use tracing::{info, error};
use tokio_postgres::types::ToSql;

#[derive(Clone)]
pub struct SearchRepository {
    pub pool: DbPool,
}

impl SearchRepository {
    pub async fn search(
        &self,
        vector: Vector,
        k: i64,
    ) -> Result<Vec<(String, f64)>, AppError> {

        info!("🗄️ Starting DB vector search");

        let start = std::time::Instant::now();

        // 🔹 Get connection
        let conn = self.pool.get().await
            .map_err(|e| {
                error!("❌ Failed to get DB connection: {}", e);
                AppError::EmbeddingError(e.to_string())
            })?;

        // 🔹 Execute query
        let rows = match conn.query(
            "SELECT content, 1 - (embedding <-> $1) AS score 
             FROM documents 
             ORDER BY embedding <-> $1 
             LIMIT $2",
            &[&vector as &(dyn ToSql + Sync), &k],
        ).await {
            Ok(r) => {
                info!("✅ DB query executed successfully");
                r
            }
            Err(e) => {
                error!("❌ DB query failed: {}", e);
                return Err(AppError::DbError(e));
            }
        };

        let results: Vec<(String, f64)> =
            rows.iter().map(|r| (r.get(0), r.get(1))).collect();

        info!(
            "📊 Retrieved {} results | took: {:?}",
            results.len(),
            start.elapsed()
        );

        Ok(results)
    }
}