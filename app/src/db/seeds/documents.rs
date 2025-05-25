use crate::db::{DbPool, repositories::DocumentRepository};
use crate::db::models::document::CreateDocument;
use serde_json::{Value, from_str};
use std::fs;
use std::path::Path;
use std::error::Error;
use tracing::{info, error};

/// Seed the database with document data
pub async fn seed(pool: DbPool) -> Result<(), Box<dyn Error>> {
    let json_path = "app/src/data/dashboard/data.json";
    
    // Check if the file exists
    if !Path::new(json_path).exists() {
        return Err(format!("JSON file not found: {}", json_path).into());
    }
    
    // Read the JSON file
    let json_content = fs::read_to_string(json_path)
        .map_err(|e| format!("Failed to read JSON file: {}", e))?;
    
    // Parse the JSON content
    let documents: Vec<Value> = from_str(&json_content)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    // Create document repository
    let repo = DocumentRepository::new(pool);
    
    info!("Seeding {} documents", documents.len());
    
    // Insert each document into the database
    for doc in documents {
        let create_doc = CreateDocument {
            header: doc["header"].as_str().unwrap_or("").to_string(),
            type_name: doc["type"].as_str().unwrap_or("").to_string(),
            status: doc["status"].as_str().unwrap_or("").to_string(),
            target: doc["target"].as_str().unwrap_or("").to_string(),
            limit_value: doc["limit"].as_str().unwrap_or("").to_string(),
            reviewer: doc["reviewer"].as_str().unwrap_or("").to_string(),
        };
        
        // Skip inserting if we already have a document with the same header
        let existing_docs = repo.get_all().await
            .map_err(|e| format!("Failed to query documents: {}", e))?;
            
        if existing_docs.iter().any(|d| d.header == create_doc.header) {
            info!("Document '{}' already exists, skipping", create_doc.header);
            continue;
        }
        
        // Insert the document
        match repo.create(create_doc.clone()).await {
            Ok(_) => info!("Inserted document: {}", create_doc.header),
            Err(e) => error!("Failed to insert document '{}': {}", create_doc.header, e),
        }
    }
    
    Ok(())
}
