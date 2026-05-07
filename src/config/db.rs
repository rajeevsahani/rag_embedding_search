use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use tracing::{info, error, debug};

pub type DbPool = Pool<PostgresConnectionManager<NoTls>>;

pub async fn create_pool(db_url: &str) -> DbPool {
    info!("🛠️ Initializing database connection pool");

    debug!("DB URL: {}", db_url);

    let start = std::time::Instant::now();

    let manager = match PostgresConnectionManager::new_from_stringlike(db_url, NoTls) {
        Ok(m) => {
            info!("✅ Postgres connection manager created");
            m
        }
        Err(e) => {
            error!("❌ Failed to create Postgres manager: {}", e);
            panic!("Cannot start app without DB");
        }
    };

    let pool = match Pool::builder().build(manager).await {
        Ok(p) => {
            info!("✅ Database pool created successfully");
            p
        }
        Err(e) => {
            error!("❌ Failed to build DB pool: {}", e);
            panic!("Cannot start app without DB");
        }
    };

    info!("⏱️ DB pool initialization took: {:?}", start.elapsed());

    pool
}