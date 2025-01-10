use actix_web::body::BoxBody;
use actix_web::body::EitherBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use colored::Colorize;
use futures::Future;
use pin_project_lite::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

pub struct RequestLogger;

impl<S> actix_web::dev::Transform<S, ServiceRequest> for RequestLogger
where
    S: actix_web::dev::Service<
        ServiceRequest,
        Response = ServiceResponse<EitherBody<BoxBody>>,
        Error = Error,
    >,
{
    type Response = ServiceResponse<EitherBody<BoxBody>>;
    type Error = Error;
    type Transform = RequestLoggerMiddleware<S>;
    type InitError = ();
    type Future = futures::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        futures::future::ready(Ok(RequestLoggerMiddleware { service }))
    }
}

pub struct RequestLoggerMiddleware<S> {
    service: S,
}

impl<S> actix_web::dev::Service<ServiceRequest> for RequestLoggerMiddleware<S>
where
    S: actix_web::dev::Service<
        ServiceRequest,
        Response = ServiceResponse<EitherBody<BoxBody>>,
        Error = Error,
    >,
{
    type Response = ServiceResponse<EitherBody<BoxBody>>;
    type Error = Error;
    type Future = RequestLoggerFuture<S::Future>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let method = req.method().to_string();
        let uri = req.uri().to_string();

        RequestLoggerFuture {
            future: self.service.call(req),
            start,
            method,
            uri,
        }
    }
}

pin_project! {
  pub struct RequestLoggerFuture<F> {
      #[pin]
      future: F,
      start: Instant,
      method: String,
      uri: String,
  }
}

impl<F> Future for RequestLoggerFuture<F>
where
    F: Future<Output = Result<ServiceResponse<EitherBody<BoxBody>>, Error>>,
{
    type Output = Result<ServiceResponse<EitherBody<BoxBody>>, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.future.poll(cx) {
            Poll::Ready(Ok(res)) => {
                let duration = this.start.elapsed();
                let status = res.status().as_u16();
                let colored_status = match status {
                    500..=599 => status.to_string().red(),
                    400..=499 => status.to_string().yellow(),
                    300..=399 => status.to_string().blue(),
                    200..=299 => status.to_string().green(),
                    100..=199 => status.to_string().purple(),
                    _ => status.to_string().normal(),
                };

                log::info!(
                    "{} {} [{}] took {:?}",
                    this.method,
                    this.uri,
                    colored_status,
                    duration
                );
                Poll::Ready(Ok(res))
            }
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        }
    }
}
