use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

/// Represents a document in the system
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Document {
    /// Unique identifier for the document
    pub id: i32,
    
    /// Document title or header
    pub header: String,
    
    /// Type of document (e.g., "Narrative", "Technical content", "Legal")
    pub type_name: String,
    
    /// Current status of the document (e.g., "In Process", "Done")
    pub status: String,
    
    /// Target value (stored as string to match JSON format)
    pub target: String,
    
    /// Limit value (stored as string to match JSON format)
    pub limit_value: String,
    
    /// Name of the reviewer assigned to the document
    pub reviewer: String,
    
    /// When the document was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<OffsetDateTime>,
    
    /// When the document was last updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<OffsetDateTime>,
}

/// Used for creating a new document
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateDocument {
    pub header: String,
    pub type_name: String,
    pub status: String,
    pub target: String,
    pub limit_value: String,
    pub reviewer: String,
}

/// Used for updating an existing document
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDocument {
    pub header: Option<String>,
    pub type_name: Option<String>,
    pub status: Option<String>,
    pub target: Option<String>,
    pub limit_value: Option<String>,
    pub reviewer: Option<String>,
}
