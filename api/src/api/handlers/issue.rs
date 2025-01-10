use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    errors::CustomError,
    models::issue::{IssueRequest, UpdateIssueRequest},
    utils::context::{get_context_org, get_context_user_id},
};

pub async fn create_issue(
    req: HttpRequest,
    state: web::Data<AppState>,
    payload: web::Json<IssueRequest>,
) -> Result<HttpResponse, CustomError> {
    let user_id = get_context_user_id(req.clone()).await?;
    let org = get_context_org(req).await?;

    let issue = state
        .issue_service
        .create_issue(payload.into_inner(), org.id, user_id)
        .await?;

    Ok(HttpResponse::Created().json(issue))
}

pub async fn get_issue(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, CustomError> {
    let (_org_id, issue_id) = path.into_inner();

    let issue = state.issue_service.get_issue(issue_id).await?;
    Ok(HttpResponse::Ok().json(issue))
}

pub async fn delete_issue(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, CustomError> {
    let (_org_id, issue_id) = path.into_inner();

    state.issue_service.delete_issue(issue_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn update_issue(
    state: web::Data<AppState>,
    payload: web::Json<UpdateIssueRequest>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, CustomError> {
    let (_org_id, issue_id) = path.into_inner();
    let issue = state
        .issue_service
        .update_issue(issue_id, payload.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(issue))
}

pub async fn get_issues(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, CustomError> {
    let org_id = path.into_inner();
    let issues = state.issue_service.get_all_by_org_id(org_id).await?;
    Ok(HttpResponse::Ok().json(issues))
}
