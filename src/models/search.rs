use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub k: Option<i64>,
}

#[derive(Serialize)]
pub struct SearchResult {
    pub content: String,
    pub score: f64,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
}