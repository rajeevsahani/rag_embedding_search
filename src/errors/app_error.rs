use thiserror::Error;
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DbError(#[from] tokio_postgres::Error),

    #[error("Embedding error: {0}")]
    EmbeddingError(String),
}
