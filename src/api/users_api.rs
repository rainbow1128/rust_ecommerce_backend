use actix_web::{get, post, web, HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use mongodb::{bson::doc, Collection};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::models::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    email: String,
    password: String,
}

#[get("/api/data")]
pub async fn get_data(
    web::Json(data): web::Json<serde_json::Value>,
    db: web::Data<Collection<User>>,
) -> impl Responder {
    let email = data["email"].as_str().unwrap_or_default();

    let query = doc! {"email":email};
    let result = db.find_one(query, None).await;

    match result {
        Ok(Some(doc)) => {
            let id_string = doc.id.map(|id| id.to_string());
            let response_data = json!({
                "id":id_string,
                "full_name": doc.full_name,
                "email": doc.email,
                "phone_number": doc.phone_number,
            });

            HttpResponse::Ok().json(response_data)
        }
        Ok(None) => HttpResponse::NotFound().body("Document not found"),
        Err(_) => HttpResponse::InternalServerError().body("Error finding document"),
    }
}

#[post("/api/users")]
pub async fn register(
    web::Json(user): web::Json<User>,
    db: web::Data<Collection<User>>,
) -> impl Responder {
    let hashed_result = hash_password(&user.password.as_str()).await.unwrap();

    let user_with_hashed_password = User {
        password: hashed_result,
        ..user
    };

    let result = db.insert_one(user_with_hashed_password, None).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User created successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create user"),
    }
}

#[post("/api/login")]
pub async fn login(
    web::Json(LoginUser { email, password }): web::Json<LoginUser>,
    db: web::Data<Collection<User>>,
) -> impl Responder {
    let user = db.find_one(doc! {"email": email}, None).await;

    match user {
        Ok(Some(doc)) => {
            let verified = verify_password(&password, doc.password).await.unwrap();
            if !verified {
                HttpResponse::NotFound().body("Invalid Credentials")
            } else {
                HttpResponse::Ok().body("Documentfound")
            }
        }
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(_) => HttpResponse::InternalServerError().body("Error Occurred!"),
    }
}

async fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

async fn verify_password(
    password: &str,
    hashed_password: String,
) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(&hashed_password)?;
    let result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);

    Ok(result.is_ok())
}
