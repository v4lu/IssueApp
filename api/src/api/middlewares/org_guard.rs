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
    models::context::{OrgContext, UserContext},
};

#[derive(Debug, Clone)]
pub struct OrgGuard;

impl<S, B> Transform<S, ServiceRequest> for OrgGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = OrgMiddleware<S>;
    type InitError = ();
    type Future = futures::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        futures::future::ready(Ok(OrgMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct OrgMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for OrgMiddleware<S>
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

        Box::pin(async move {
            let state = req
                .app_data::<web::Data<AppState>>()
                .ok_or_else(|| CustomError::InternalServerError)?;

            let user_context = req
                .extensions()
                .get::<UserContext>()
                .ok_or(CustomError::Unauthorized)?
                .clone();

            let org_id = req
                .match_info()
                .get("org_id")
                .ok_or_else(|| CustomError::NotFound("Organization ID not found".to_string()))?
                .to_string();
            let org_id = uuid::Uuid::parse_str(&org_id).map_err(|_| CustomError::BadRequest)?;

            let user_id = user_context
                .user_id
                .as_ref()
                .ok_or(CustomError::Unauthorized)?;

            let user = state.auth_service.get_session(user_id.clone()).await?;

            let org = state
                .org_service
                .get_org(org_id, user.id)
                .await
                .map_err(|_| CustomError::NotFound("Organization not found".to_string()))?;

            let org_context = OrgContext { org: Some(org) };

            if let Some(org_id) = org_context.org.as_ref().map(|o| &o.id) {
                log::info!("OrgContext: {}", org_id);
            }

            req.extensions_mut().insert(org_context);

            service.call(req).await.map_err(Into::into)
        })
    }
}
