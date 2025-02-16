use std::sync::Arc;

use crate::{errors::CustomError, models::docs::{DocPagableResponse, DocPageRequest, DocRequest, DocResponse, UpdateDocRequest}, repositories::docs::DocsRepository};

use super::user::UserService;

pub struct DocService {
  user_service: Arc<UserService>,
  docs_repo: DocsRepository,
}

impl DocService {
  pub fn new(user_service: Arc<UserService>, pool: sqlx::PgPool) -> Self {
    Self { 
      user_service,
            docs_repo: DocsRepository::new(pool.clone()) }
     
  }

  pub async fn create_doc(&self, session_id: uuid::Uuid, org_id: uuid::Uuid, request: DocRequest) -> Result<DocResponse, CustomError> {
    let user = self.user_service.get_user(session_id).await?;

    let doc = self.docs_repo.insert(org_id, session_id, request).await
     .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

    Ok(DocResponse {
      doc,
      creator: user,
    })
  }

  pub async fn get_doc(&self, org_id: uuid::Uuid, doc_id: uuid::Uuid) -> Result<DocResponse, CustomError> {
    let doc = self.docs_repo.get(org_id, doc_id).await
     .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

    let creator = self.user_service.get_user(doc.creator_id).await
     .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

    Ok(DocResponse {
      doc,
      creator,
    })
  }

  pub async fn delete_doc(&self, org_id: uuid::Uuid, doc_id: uuid::Uuid) -> Result<(), CustomError> {
    self.docs_repo.delete(org_id, doc_id).await
     .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

    Ok(())
  }

  pub async fn update_doc(&self, org_id: uuid::Uuid, doc_id: uuid::Uuid, request: UpdateDocRequest) -> Result<DocResponse, CustomError> {
    let doc = self.docs_repo.update(org_id, doc_id, request).await
     .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

    let creator = self.user_service.get_user(doc.creator_id).await
     .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

    Ok(DocResponse {
      doc,
      creator,
    })
  }

  pub async fn get_all_org(&self, org_id: uuid::Uuid, request: DocPageRequest) -> Result<DocPagableResponse, CustomError> {
    let docs = self.docs_repo.get_all(org_id, request).await
     .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

    let mut docs_response = vec![];

    for doc in docs.docs {
      let creator = self.user_service.get_user(doc.creator_id).await
       .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

      docs_response.push(DocResponse {
        doc,
        creator,
      });
    }

    Ok(DocPagableResponse {
      docs: docs_response,
      total: docs.total,
      page: docs.page,
      page_size: docs.page_size,
      last_page: docs.last_page,
    })
  }
}

