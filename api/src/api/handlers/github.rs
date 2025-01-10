use actix_web::{web, HttpRequest, HttpResponse};

use crate::{
    app_state::AppState,
    errors::CustomError,
    models::github::{CreateIssueRequest, UpdateIssueRequest},
    services::github::GithubService,
};

pub async fn get_repos(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let code = req
        .query_string()
        .split('&')
        .find(|param| param.starts_with("code="))
        .map(|code| code.replace("code=", ""))
        .ok_or(CustomError::InternalServerError)?;

    let access_token = state.oauth_service.get_github_access_token(code).await?;
    let gh_service = GithubService::new(&access_token.access_token)?;
    let repos = gh_service.get_org_repos("v4lu").await?;
    Ok(HttpResponse::Ok().json(repos))
}

pub async fn get_repository(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let code = req
        .query_string()
        .split('&')
        .find(|param| param.starts_with("code="))
        .map(|code| code.replace("code=", ""))
        .ok_or(CustomError::InternalServerError)?;

    let (owner, repo_name) = path.into_inner();
    let access_token = state.oauth_service.get_github_access_token(code).await?;
    let gh_service = GithubService::new(&access_token.access_token)?;
    let repo = gh_service.get_repository(&owner, &repo_name).await?;
    Ok(HttpResponse::Ok().json(repo))
}

pub async fn get_repository_issues(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let code = req
        .query_string()
        .split('&')
        .find(|param| param.starts_with("code="))
        .map(|code| code.replace("code=", ""))
        .ok_or(CustomError::InternalServerError)?;

    let (owner, repo_name) = path.into_inner();
    let access_token = state.oauth_service.get_github_access_token(code).await?;
    let gh_service = GithubService::new(&access_token.access_token)?;
    let issues = gh_service.get_repository_issues(&owner, &repo_name).await?;
    Ok(HttpResponse::Ok().json(issues))
}

pub async fn get_issue(
    req: HttpRequest,
    path: web::Path<(String, String, u64)>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let code = req
        .query_string()
        .split('&')
        .find(|param| param.starts_with("code="))
        .map(|code| code.replace("code=", ""))
        .ok_or(CustomError::InternalServerError)?;

    let (owner, repo_name, issue_number) = path.into_inner();
    let access_token = state.oauth_service.get_github_access_token(code).await?;
    let gh_service = GithubService::new(&access_token.access_token)?;
    let issue = gh_service
        .get_issue(&owner, &repo_name, issue_number)
        .await?;
    Ok(HttpResponse::Ok().json(issue))
}

pub async fn create_issue(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    state: web::Data<AppState>,
    payload: web::Json<CreateIssueRequest>,
) -> Result<HttpResponse, CustomError> {
    let code = req
        .query_string()
        .split('&')
        .find(|param| param.starts_with("code="))
        .map(|code| code.replace("code=", ""))
        .ok_or(CustomError::InternalServerError)?;

    let (owner, repo_name) = path.into_inner();
    let access_token = state.oauth_service.get_github_access_token(code).await?;
    let gh_service = GithubService::new(&access_token.access_token)?;
    let issue = gh_service
        .create_issue(
            &owner,
            &repo_name,
            &payload.title,
            payload.body.clone(),
            payload.assignees.clone(),
            payload.labels.clone(),
        )
        .await?;
    Ok(HttpResponse::Created().json(issue))
}

pub async fn update_issue(
    req: HttpRequest,
    path: web::Path<(String, String, u64)>,
    state: web::Data<AppState>,
    payload: web::Json<UpdateIssueRequest>,
) -> Result<HttpResponse, CustomError> {
    let code = req
        .query_string()
        .split('&')
        .find(|param| param.starts_with("code="))
        .map(|code| code.replace("code=", ""))
        .ok_or(CustomError::InternalServerError)?;

    let (owner, repo_name, issue_number) = path.into_inner();
    let access_token = state.oauth_service.get_github_access_token(code).await?;
    let gh_service = GithubService::new(&access_token.access_token)?;

    let issue = gh_service
        .update_issue(
            &owner,
            &repo_name,
            issue_number,
            payload.title.clone(),
            payload.body.clone(),
            payload.state.clone(),
            payload.assignees.clone(),
            payload.labels.clone(),
        )
        .await?;
    Ok(HttpResponse::Ok().json(issue))
}
