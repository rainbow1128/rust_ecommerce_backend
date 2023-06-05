use std::{
    env,
    future::{ready, Ready},
};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::api::users_api::Claims;

pub struct SayHi;

impl<S, B> Transform<S, ServiceRequest> for SayHi
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SayHiMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SayHiMiddleware { service }))
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SayHiMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|header_value| header_value.to_str().ok())
            .and_then(|header_value| {
                if header_value.starts_with("Bearer ") {
                    Some(header_value.trim_start_matches("Bearer ").to_string())
                } else {
                    None
                }
            });

        // let verification = verify_jwt(&token.unwrap().to_string());

        let fut = match verify_jwt(&token.unwrap().to_string()) {
            Ok(claims) => {
                println!("{:?}", claims);
                self.service.call(req)
            }
            Err(_) => todo!(),
        };
        Box::pin(async move {
            let res = fut.await?;

            Ok(res)
        })
    }
}

fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set in .env");
    let decoding_key = DecodingKey::from_secret(secret_key.as_bytes());
    let validation = Validation::default();

    let decoded_token = decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(decoded_token.claims)
}
