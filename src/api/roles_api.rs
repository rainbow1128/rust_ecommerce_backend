use actix_web::{get, web, HttpResponse, Responder};
use mongodb::{bson::doc, Collection};

use crate::models::roles::Roles;

//api/roles/create
pub async fn create_role(
    web::Json(roles): web::Json<Roles>,
    db: web::Data<Collection<Roles>>,
) -> impl Responder {
    let result = db.insert_one(roles, None).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Roles created successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create user"),
    }
}

#[get("/api/roles/create-admin")]
pub async fn create_admin_role(db: web::Data<Collection<Roles>>) -> impl Responder {
    let admin_check = db.find_one(doc! {"role_name": "Administrator"}, None).await;

    if admin_check.unwrap().is_some() {
        return HttpResponse::InternalServerError().body("Already Exists");
    }

    let role = Roles {
        id: None,
        role_name: "Administrator".to_string(),
        models: None,
    };

    let result = db.insert_one(role, None).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Roles created successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create user"),
    }
}
