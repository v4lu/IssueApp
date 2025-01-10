use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    errors::CustomError,
    models::comment::{CommentRequest, UpdateCommentRequest},
    utils::context::get_context_user_id,
};

pub async fn create_comment(
    req: HttpRequest,
    comment_data: web::Json<CommentRequest>,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, CustomError> {
    let user_id = get_context_user_id(req).await?;

    let comment = state
        .comment_service
        .create_comment(comment_data.into_inner(), *path, user_id)
        .await?;

    Ok(HttpResponse::Ok().json(comment))
}

pub async fn update_comment(
    req: HttpRequest,
    comment_data: web::Json<UpdateCommentRequest>,
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, CustomError> {
    let user_id = get_context_user_id(req).await?;

    // 1 org_id, 2 comment_id
    let (_, comment_id) = path.into_inner();

    let comment = state
        .comment_service
        .update_comment(comment_id, user_id, comment_data.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(comment))
}

pub async fn delete_comment(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, CustomError> {
    let user_id = get_context_user_id(req).await?;

    // 1 org_id, 2 comment_id
    let (_, comment_id) = path.into_inner();

    state
        .comment_service
        .delete_comment(comment_id, user_id)
        .await?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn get_comment(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, CustomError> {
    // 1 org_id, 2 comment_id
    let (_, comment_id) = path.into_inner();

    let comment = state.comment_service.get_comment(comment_id).await?;

    Ok(HttpResponse::Ok().json(comment))
}

pub async fn get_comments(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, CustomError> {
    // 1 org_id, 2 comment_owner_id
    let (_, comment_owner_id) = path.into_inner();

    let comments = state
        .comment_service
        .get_comments_by_owner_id(comment_owner_id)
        .await?;

    Ok(HttpResponse::Ok().json(comments))
}
