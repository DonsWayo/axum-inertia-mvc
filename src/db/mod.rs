pub mod models;
pub mod repositories;
pub mod seeds;
pub mod migrations;
pub mod error;
pub mod connection;

// Re-export commonly used types
pub use error::DatabaseError;
pub use connection::{DbPool, init_db_pool};
