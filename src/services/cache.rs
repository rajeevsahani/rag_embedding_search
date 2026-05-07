use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tracing::{info, debug};

#[derive(Clone)]
pub struct EmbeddingCache {
    store: Arc<RwLock<HashMap<String, Vec<f32>>>>,
}

impl EmbeddingCache {
    pub fn new() -> Self {
        info!("🧠 Initializing embedding cache");

        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get(&self, key: &str) -> Option<Vec<f32>> {
        debug!("🔍 Cache lookup for key: {}", key);

        let result = self.store.read().await.get(key).cloned();

        if result.is_some() {
            info!("⚡ Cache HIT");
        } else {
            info!("❌ Cache MISS");
        }

        result
    }

    pub async fn set(&self, key: String, val: Vec<f32>) {
        debug!("💾 Storing embedding in cache");

        let mut store = self.store.write().await;
        store.insert(key, val);

        info!("✅ Cache updated | size={}", store.len());
    }
}