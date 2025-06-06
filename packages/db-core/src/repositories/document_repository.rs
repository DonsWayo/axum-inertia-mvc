use crate::{DbPool, DbError};
use crate::models::document::{Document, CreateDocument, UpdateDocument};
use chrono::Utc;

/// Repository for document-related database operations
pub struct DocumentRepository {
    pool: DbPool,
}

impl DocumentRepository {
    /// Create a new DocumentRepository instance
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
    
    /// Get all documents
    pub async fn get_all(&self) -> Result<Vec<Document>, DbError> {
        sqlx::query_as::<_, Document>(
            "SELECT id, header, type_name, status, target, limit_value, reviewer, created_at, updated_at FROM documents"
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))
    }
    
    /// Get a document by ID
    pub async fn get_by_id(&self, id: i32) -> Result<Option<Document>, DbError> {
        sqlx::query_as::<_, Document>(
            "SELECT id, header, type_name, status, target, limit_value, reviewer, created_at, updated_at FROM documents WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))
    }
    
    /// Create a new document
    pub async fn create(&self, document: CreateDocument) -> Result<Document, DbError> {
        let now = Utc::now();
        
        sqlx::query_as::<_, Document>(
            "INSERT INTO documents (header, type_name, status, target, limit_value, reviewer, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             RETURNING id, header, type_name, status, target, limit_value, reviewer, created_at, updated_at"
        )
        .bind(&document.header)
        .bind(&document.type_name)
        .bind(&document.status)
        .bind(&document.target)
        .bind(&document.limit_value)
        .bind(&document.reviewer)
        .bind(now)
        .bind(now)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))
    }
    
    /// Update an existing document
    pub async fn update(&self, id: i32, document: UpdateDocument) -> Result<Document, DbError> {
        // First check if the document exists
        let existing = self.get_by_id(id).await?;
        if existing.is_none() {
            return Err(DbError::QueryError(format!("Document with ID {} not found", id)));
        }
        
        let existing = existing.unwrap();
        let now = Utc::now();
        
        // Build the update query
        let updated_document = sqlx::query_as::<_, Document>(
            "UPDATE documents SET 
                header = $1, 
                type_name = $2, 
                status = $3,
                target = $4,
                limit_value = $5,
                reviewer = $6,
                updated_at = $7
             WHERE id = $8
             RETURNING id, header, type_name, status, target, limit_value, reviewer, created_at, updated_at"
        )
        .bind(document.header.unwrap_or(existing.header))
        .bind(document.type_name.unwrap_or(existing.type_name))
        .bind(document.status.unwrap_or(existing.status))
        .bind(document.target.unwrap_or(existing.target))
        .bind(document.limit_value.unwrap_or(existing.limit_value))
        .bind(document.reviewer.unwrap_or(existing.reviewer))
        .bind(now)
        .bind(id)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;
        
        Ok(updated_document)
    }
    
    /// Delete a document by ID
    pub async fn delete(&self, id: i32) -> Result<(), DbError> {
        let result = sqlx::query("DELETE FROM documents WHERE id = $1")
            .bind(id)
            .execute(&*self.pool)
            .await
            .map_err(|e| DbError::QueryError(e.to_string()))?;
            
        if result.rows_affected() == 0 {
            return Err(DbError::QueryError(format!("Document with ID {} not found", id)));
        }
        
        Ok(())
    }
    
    /// Get documents by status
    pub async fn get_by_status(&self, status: &str) -> Result<Vec<Document>, DbError> {
        sqlx::query_as::<_, Document>(
            "SELECT id, header, type_name, status, target, limit_value, reviewer, created_at, updated_at 
             FROM documents WHERE status = $1"
        )
        .bind(status)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))
    }
    
    /// Get documents by type
    pub async fn get_by_type(&self, type_name: &str) -> Result<Vec<Document>, DbError> {
        sqlx::query_as::<_, Document>(
            "SELECT id, header, type_name, status, target, limit_value, reviewer, created_at, updated_at 
             FROM documents WHERE type_name = $1"
        )
        .bind(type_name)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))
    }
    
    /// Get documents by reviewer
    pub async fn get_by_reviewer(&self, reviewer: &str) -> Result<Vec<Document>, DbError> {
        sqlx::query_as::<_, Document>(
            "SELECT id, header, type_name, status, target, limit_value, reviewer, created_at, updated_at 
             FROM documents WHERE reviewer = $1"
        )
        .bind(reviewer)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))
    }
}
