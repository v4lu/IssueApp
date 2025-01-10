use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    errors::CustomError,
    models::org::{
        CreateOrgRequest, InviteOrgMemberRequest, MemberRole, UpdateOrgMemberRequest,
        UpdateOrgRequest,
    },
    utils::context::{get_context_org, get_context_user_id},
};

pub async fn create_org(
    req: HttpRequest,
    org_data: web::Json<CreateOrgRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let user_id = get_context_user_id(req).await?;

    let org = state
        .org_service
        .create_org(org_data.into_inner(), user_id)
        .await?;

    Ok(HttpResponse::Ok().json(org))
}

pub async fn list_organizations(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let user_id = get_context_user_id(req).await?;

    let orgs = state.org_service.list_user_orgs(user_id).await?;
    Ok(HttpResponse::Ok().json(orgs))
}

pub async fn list_user_invites(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let user_id = get_context_user_id(req).await?;

    let invites = state.org_service.list_user_invites(user_id).await?;
    Ok(HttpResponse::Ok().json(invites))
}

pub async fn accept_invite(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<(String, Uuid)>,
) -> Result<HttpResponse, CustomError> {
    let user_id = get_context_user_id(req).await?;

    state
        .org_service
        .accept_org_invite(path.1, user_id, path.0.clone())
        .await?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn cancel_invite(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, CustomError> {
    state
        .org_service
        .decline_org_invite(path.into_inner())
        .await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn get_org(req: HttpRequest) -> Result<HttpResponse, CustomError> {
    let org = get_context_org(req).await?;
    Ok(HttpResponse::Ok().json(org))
}

pub async fn update_org(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateOrgRequest>,
) -> Result<HttpResponse, CustomError> {
    let org = state
        .org_service
        .update_org(path.into_inner(), payload.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(org))
}

pub async fn delete_org(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, CustomError> {
    state.org_service.delete_org(path.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn invite_user(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    payload: web::Json<InviteOrgMemberRequest>,
) -> Result<HttpResponse, CustomError> {
    let user_id = get_context_user_id(req).await?;

    let invite = state
        .org_service
        .invite_user_to_org(
            path.into_inner(),
            payload.into_inner(),
            MemberRole::Member,
            user_id,
        )
        .await?;

    Ok(HttpResponse::Created().json(invite))
}

pub async fn list_org_members(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, CustomError> {
    let members = state
        .org_service
        .list_org_members(path.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(members))
}

pub async fn list_org_invites(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, CustomError> {
    let invites = state
        .org_service
        .list_org_invites(path.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(invites))
}

pub async fn update_member_role(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
    payload: web::Json<UpdateOrgMemberRequest>,
) -> Result<HttpResponse, CustomError> {
    let (org_id, member_id) = path.into_inner();

    state
        .org_service
        .change_member_role(org_id, member_id, payload.into_inner().role.unwrap())
        .await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn remove_member(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, CustomError> {
    let (org_id, member_id) = path.into_inner();
    state.org_service.remove_member(org_id, member_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn get_session_role(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, CustomError> {
    let user_id = get_context_user_id(req).await?;
    let org_id = path.into_inner();

    let role = state.org_service.check_user_role(org_id, user_id).await?;

    Ok(HttpResponse::Ok().json(role))
}
