use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database connection error: {0}")]
    ConnectionError(String),
    
    #[error("Database query error: {0}")]
    QueryError(String),
    
    #[error("Database migration error: {0}")]
    MigrationError(String),

    #[error("Resource not found")]
    NotFound,
}

impl From<sqlx::Error> for DbError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => DbError::NotFound,
            _ => DbError::QueryError(err.to_string()),
        }
    }
}