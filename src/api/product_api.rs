use actix_web::{post, web, HttpResponse, Responder};
use mongodb::Collection;

use crate::models::product_model::Product;

#[post("/api/product/create")]
pub async fn create_product(
    web::Json(product): web::Json<Product>,
    db: web::Data<Collection<Product>>,
) -> impl Responder {
    let result = db.insert_one(product, None).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Product added successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to add product"),
    }
}
