use actix_web::{web, HttpRequest, HttpResponse};

use crate::{app_state::AppState, errors::CustomError, models::docs::{DocPageRequest, DocQueryParams, DocRequest, UpdateDocRequest},  utils::context::{get_context_org, get_context_user_id}};

pub async fn create_doc(
    req: HttpRequest,
    state: web::Data<AppState>,
    payload: web::Json<DocRequest>,
) -> Result<HttpResponse, CustomError> {
    let user_id = get_context_user_id(req.clone()).await?;
    let org = get_context_org(req).await?;

    let issue = state
        .doc_service
        .create_doc(user_id, org.id, payload.into_inner())
        .await?;

    Ok(HttpResponse::Created().json(issue))
}

pub async fn get_doc(
    state: web::Data<AppState>,
    path: web::Path<(uuid::Uuid, uuid::Uuid)>,
) -> Result<HttpResponse, CustomError> {
    let(org_id, doc_id) = path.into_inner();

    let doc = state.doc_service.get_doc(org_id, doc_id).await?;
    Ok(HttpResponse::Ok().json(doc))
}

pub async fn delete_doc(
    state: web::Data<AppState>,
    path: web::Path<(uuid::Uuid, uuid::Uuid)>,
) -> Result<HttpResponse, CustomError> {
    let(org_id, doc_id) = path.into_inner();

    state.doc_service.delete_doc(org_id, doc_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn update_doc(
    state: web::Data<AppState>,
    payload: web::Json<UpdateDocRequest>,
    path: web::Path<(uuid::Uuid, uuid::Uuid)>
) -> Result<HttpResponse, CustomError> {
    let(org_id, doc_id) = path.into_inner();
    let doc = state.doc_service.update_doc(org_id, doc_id, payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(doc))
}

pub async fn get_all_org_docs(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let org = get_context_org(req.clone()).await?;
     let query = web::Query::<DocQueryParams>::from_query(req.query_string())
        .map_err(|_| CustomError::BadRequest)?;

      let request = DocPageRequest {
        order: query.order.clone(),
        sort: query.sort.clone(),
        search: query.search.clone(),
        page: query.page.unwrap_or(1),
        page_size: query.page_size.unwrap_or(20),
       
      };


    let docs = state.doc_service.get_all_org(org.id, request).await?;
    Ok(HttpResponse::Ok().json(docs))
}
