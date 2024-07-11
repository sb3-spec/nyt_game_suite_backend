use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error("Failed to apply migrations")]
    MigrationsFailed(#[from] sqlx::migrate::MigrateError),

    #[error("Not authorized to perform this action")]
    NotAuthorized,

    #[error(transparent)]
    RediError(#[from] redis::RedisError),
}
