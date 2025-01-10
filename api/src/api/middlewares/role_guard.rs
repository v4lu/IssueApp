use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage,
};
use futures::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use crate::{
    app_state::AppState,
    errors::CustomError,
    models::{
        context::{OrgContext, UserContext},
        org::MemberRole,
    },
};

pub struct RoleGuard {
    allowed_roles: Vec<MemberRole>,
}

impl RoleGuard {
    pub fn new(roles: Vec<MemberRole>) -> Self {
        Self {
            allowed_roles: roles,
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RoleGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RoleMiddleware<S>;
    type InitError = ();
    type Future = futures::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        futures::future::ready(Ok(RoleMiddleware {
            service: Rc::new(service),
            allowed_roles: self.allowed_roles.clone(),
        }))
    }
}

pub struct RoleMiddleware<S> {
    service: Rc<S>,
    allowed_roles: Vec<MemberRole>,
}

impl<S, B> Service<ServiceRequest> for RoleMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let allowed_roles = self.allowed_roles.clone();
        log::info!("Allowed Roles: {:?}", allowed_roles);

        Box::pin(async move {
            let state = req
                .app_data::<web::Data<AppState>>()
                .ok_or(CustomError::InternalServerError)?;

            let org = {
                let extensions = req.extensions();
                let org_context = extensions
                    .get::<OrgContext>()
                    .ok_or(CustomError::Unauthorized)?;
                org_context
                    .org
                    .as_ref()
                    .ok_or(CustomError::Unauthorized)?
                    .clone()
            };

            let user_context = req
                .extensions()
                .get::<UserContext>()
                .ok_or(CustomError::Unauthorized)?
                .clone();

            let user_id = user_context
                .user_id
                .as_ref()
                .ok_or(CustomError::Unauthorized)?;

            let user = state.auth_service.get_session(user_id.clone()).await?;

            let user_role = state
                .org_service
                .check_user_role(org.id, user.id)
                .await
                .map_err(|_| CustomError::Forbidden)?;

            log::info!("User Role: {:?}", user_role);

            if !allowed_roles.contains(&user_role) {
                return Err(CustomError::Forbidden.into());
            }

            service.call(req).await
        })
    }
}
