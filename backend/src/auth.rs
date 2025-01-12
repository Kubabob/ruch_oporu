use actix_web::dev::Service;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use futures::future::{ok, Ready};
use futures::FutureExt;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::task::{Context, Poll};
use actix_web::error::ErrorUnauthorized;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct AuthMiddleware<S> {
    pub service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error, Future = futures::future::BoxFuture<'static, Result<ServiceResponse<B>, Error>>> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), actix_web::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|header| header.strip_prefix("Bearer "))
            .map(|token| token.to_string());

        if let Some(token) = token {
            let decoding_key = DecodingKey::from_secret("your_secret_key".as_ref());
            let validation = Validation::default();

            match decode::<Claims>(&token, &decoding_key, &validation) {
                Ok(_) => {
                    let fut = self.service.call(req);
                    Box::pin(async move { fut.await })
                }
                Err(_) => {
                    let response = req.error_response(ErrorUnauthorized("Invalid token"));
                    Box::pin(async { Ok(ServiceResponse::new(req.into_parts().0, response.map_into_boxed_body())) })
                }
            }
        } else {
            Box::pin(async { 
                let response = req.error_response(ErrorUnauthorized("Missing token"));
                Ok(ServiceResponse::new(req.into_parts().0, response.map_into_boxed_body().into()))
            })
        }
    }
}

