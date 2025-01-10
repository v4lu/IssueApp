use actix_web::{web, HttpResponse};

use crate::{
    app_state::AppState,
    errors::CustomError,
    models::user_preferences::{UserPreferenceRequest, UserPreferenceUpdateRequest},
    utils::context::get_context_user_id,
};

pub async fn create_user_preferences(
    req: actix_web::HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let user_id = get_context_user_id(req).await?;

    let up_req = UserPreferenceRequest { user_id };
    let user_preferences = state
        .user_preferences_service
        .create_user_preference(up_req)
        .await
        .map_err(|_e| CustomError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(user_preferences))
}

pub async fn get_user_preferences(
    req: actix_web::HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let user_id = get_context_user_id(req).await?;

    let user_preferences = state
        .user_preferences_service
        .get_user_preferences(user_id)
        .await
        .map_err(|_e| CustomError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(user_preferences))
}

pub async fn update_user_preference(
    req: actix_web::HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<UserPreferenceUpdateRequest>,
) -> Result<HttpResponse, CustomError> {
    let user_id = get_context_user_id(req).await?;

    let user_preferences = state
        .user_preferences_service
        .update_user_preference(user_id, body.into_inner())
        .await
        .map_err(|_e| CustomError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(user_preferences))
}
