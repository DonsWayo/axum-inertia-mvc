use db_core::{
    models::document::{Document, CreateDocument, UpdateDocument},
    repositories::DocumentRepository,
    error::DbError,
    DbPool,
};

pub struct DocumentService {
    pool: DbPool,
}

impl DocumentService {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Document>, DbError> {
        let repo = DocumentRepository::new(self.pool.clone());
        repo.get_all().await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Document, DbError> {
        let repo = DocumentRepository::new(self.pool.clone());
        match repo.get_by_id(id).await {
            Ok(Some(doc)) => Ok(doc),
            Ok(None) => Err(DbError::NotFound),
            Err(e) => Err(e)
        }
    }

    pub async fn create(&self, document: CreateDocument) -> Result<Document, DbError> {
        let repo = DocumentRepository::new(self.pool.clone());
        repo.create(document).await
    }

    pub async fn update(&self, id: i32, document: UpdateDocument) -> Result<Document, DbError> {
        let repo = DocumentRepository::new(self.pool.clone());
        repo.update(id, document).await
    }

    pub async fn delete(&self, id: i32) -> Result<(), DbError> {
        let repo = DocumentRepository::new(self.pool.clone());
        repo.delete(id).await
    }
}