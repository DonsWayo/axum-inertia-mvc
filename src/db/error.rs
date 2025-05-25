use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Database connection error: {0}")]
    ConnectionError(String),
    
    #[error("Database query error: {0}")]
    QueryError(String),
    
    #[error("Database migration error: {0}")]
    MigrationError(String),

    #[error("Resource not found")]
    NotFound,
}
