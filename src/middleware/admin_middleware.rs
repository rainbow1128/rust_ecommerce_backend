use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};

use crate::models::user::User;

// Middleware for user authentication

pub struct AdminMiddleware {
    pub db: Data<Collection<User>>,
}

impl<S: 'static, B> Transform<S, ServiceRequest> for AdminMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AdminMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AdminMiddlewareService {
            service: Rc::new(service),
            db: self.db.clone(),
        }))
    }
}

pub struct AdminMiddlewareService<S> {
    service: Rc<S>,
    db: Data<Collection<User>>,
}

impl<S, B> Service<ServiceRequest> for AdminMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        let db = self.db.clone();
        let id = req.extensions().get::<String>().cloned();
        let id = ObjectId::parse_str(id.unwrap())
            .map_err(|_| HttpResponse::InternalServerError().body("Invalid ID format"));

        let query = doc! {"_id": id.unwrap()};

        Box::pin(async move {
            let result = db.find_one(query, None).await;

            match result {
                Ok(Some(doc)) => {
                    println!("Found document: {:?}", doc.id);
                    // Perform further operations with the retrieved document
                }
                Ok(None) => {
                    println!("Document not found");
                    // Handle the case when the document is not found
                }
                Err(err) => {
                    println!("Error occurred: {:?}", err);
                    // Handle the error case
                }
            }

            let res = svc.call(req).await?;

            Ok(res)
        })
    }
}
