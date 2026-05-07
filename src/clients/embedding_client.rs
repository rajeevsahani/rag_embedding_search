use tracing::{info, error, debug};

#[derive(Clone)]
pub struct EmbeddingClient {
    pub http: reqwest::Client,
}

impl EmbeddingClient {
    pub async fn get_embedding(&self, query: &str) -> Result<Vec<f32>, String> {
        info!("📡 Sending embedding request");

        debug!("Query: {}", query);

        let res = self.http
            .post("http://127.0.0.1:8000/embed-only")
            .json(&serde_json::json!({ "text": query }))
            .send()
            .await;

        let res = match res {
            Ok(r) => {
                info!("✅ Received response from embedding service");
                r
            }
            Err(e) => {
                error!("❌ HTTP request failed: {}", e);
                return Err(e.to_string());
            }
        };

        let json: serde_json::Value = match res.json().await {
            Ok(j) => j,
            Err(e) => {
                error!("❌ Failed to parse JSON: {}", e);
                return Err(e.to_string());
            }
        };

        debug!("Raw response: {:?}", json);

        let arr = match json["embedding"].as_array() {
            Some(a) => a,
            None => {
                error!("❌ Missing 'embedding' field in response");
                return Err("Missing embedding field".to_string());
            }
        };

        let embedding: Vec<f32> = arr
            .iter()
            .map(|v| v.as_f64().unwrap_or(0.0) as f32)
            .collect();

        info!("🎯 Embedding generated successfully (dim={})", embedding.len());

        Ok(embedding)
    }
}