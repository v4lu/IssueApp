use actix_web::{web, HttpRequest, HttpResponse};

use crate::{
    app_state::AppState,
    errors::CustomError,
    models::auth::{LoginRequest, RegisterRequest},
    utils::context::get_context_user_id,
};

pub async fn register(
    user_data: web::Json<RegisterRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let user_res = state
        .auth_service
        .register_user(user_data.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(user_res))
}

pub async fn login(
    login_data: web::Json<LoginRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let login_res = state
        .auth_service
        .login_user(login_data.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(login_res))
}

pub async fn refresh_token(
    req: actix_web::HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let refresh_token = req
        .headers()
        .get("RefreshTokenX")
        .and_then(|header| header.to_str().ok())
        .unwrap_or_default();

    let token_res = state.auth_service.refresh_token(refresh_token).await?;

    Ok(HttpResponse::Ok().json(token_res))
}

pub async fn session(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let user_id = get_context_user_id(req).await?;
    let session = state.auth_service.get_session(user_id).await?;

    Ok(HttpResponse::Ok().json(session))
}
#[derive(serde::Serialize)]
struct GhResponse {
    url: reqwest::Url,
}

pub async fn init_github(state: web::Data<AppState>) -> Result<HttpResponse, CustomError> {
    let gh_url = state.oauth_service.init_github_oauth().await;
    Ok(HttpResponse::Ok().json(GhResponse { url: gh_url }))
}

pub async fn github_callback(req: HttpRequest) -> Result<HttpResponse, CustomError> {
    let code = req
        .query_string()
        .split('&')
        .find(|param| param.starts_with("code="))
        .map(|code| code.replace("code=", ""))
        .ok_or(CustomError::InternalServerError)?;

    log::info!("Github callback code: {}", code);
    let redirect_url = format!("http://localhost:3000?code={}", code);

    Ok(HttpResponse::Found()
        .append_header(("Location", redirect_url))
        .finish())
}

pub async fn login_with_github(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let code = req
        .query_string()
        .split('&')
        .find(|param| param.starts_with("code="))
        .map(|code| code.replace("code=", ""))
        .ok_or(CustomError::InternalServerError)?;

    let user_res = state.oauth_service.handle_github_callback(code).await?;
    Ok(HttpResponse::Ok().json(user_res))
}
