use actix_web::{HttpMessage, HttpRequest};
use uuid::Uuid;

use crate::{
    errors::CustomError,
    models::{
        context::{OrgContext, UserContext},
        org::Org,
    },
};

pub async fn get_context_user_id(req: HttpRequest) -> Result<Uuid, CustomError> {
    let extensions = req.extensions();
    let user_context = extensions
        .get::<UserContext>()
        .ok_or(CustomError::Unauthorized)?;

    let user_id = user_context.user_id.ok_or(CustomError::Unauthorized)?;

    Ok(user_id)
}

pub async fn get_context_org(req: HttpRequest) -> Result<Org, CustomError> {
    let extensions = req.extensions();
    let org_context = extensions
        .get::<OrgContext>()
        .ok_or(CustomError::Unauthorized)?;

    let org = org_context.org.clone().ok_or(CustomError::Unauthorized)?;

    Ok(org)
}
