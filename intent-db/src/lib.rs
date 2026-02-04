pub mod entity;
pub mod migration;
pub mod repo;

pub use sea_orm;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait;
use tracing::error;
use migration::Migrator;
use std::time::Duration;

/// Initialize database connection with robust pooling and run migrations
pub async fn init_db(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(database_url);
    opt.acquire_timeout(Duration::from_secs(15))
        .min_connections(1)
        .max_connections(20) // Limit max connections for Supabase Pooler
        .connect_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(900))
        .sqlx_logging(false) // Reduce log noise
        .test_before_acquire(true);

    let db = Database::connect(opt).await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        e
    })?;
    
    // Run migrations
    Migrator::up(&db, None).await?;
    
    Ok(db)
}
