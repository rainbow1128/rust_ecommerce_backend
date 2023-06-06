use std::env;

use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::models::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub username: String,
    pub exp: i64,
}

// api/users/me
pub async fn get_data(req: HttpRequest, db: web::Data<Collection<User>>) -> impl Responder {
    let id = req
        .extensions()
        .get::<String>()
        .cloned()
        .ok_or_else(|| HttpResponse::InternalServerError().body("Invalid request state"));

    let id = ObjectId::parse_str(id.unwrap())
        .map_err(|_| HttpResponse::InternalServerError().body("Invalid ID format"));

    let query = doc! {"_id":id.unwrap()};
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

#[post("/api/users/register")]
pub async fn register(
    web::Json(user): web::Json<User>,
    db: web::Data<Collection<User>>,
) -> impl Responder {
    let hashed_result = hash_password(user.password.as_str()).await.unwrap();

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

#[post("/api/users/login")]
pub async fn login(
    web::Json(LoginUser { email, password }): web::Json<LoginUser>,
    db: web::Data<Collection<User>>,
) -> impl Responder {
    let user = db.find_one(doc! {"email": email}, None).await;

    match user {
        Ok(Some(doc)) => {
            let verified = verify_password(&password, doc.password.clone())
                .await
                .unwrap();
            if !verified {
                HttpResponse::NotFound().body("Invalid Credentials")
            } else {
                let jwt_token = generate_jwt(&doc).expect("Failed to generate JWT");

                HttpResponse::Ok().json(json!({
                    "access_token":jwt_token,
                }))
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

// Generate a JWT for the user
fn generate_jwt(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    // Define the payload of the JWT
    let claims = Claims {
        id: user.id.unwrap().to_string(),
        username: user.username.clone(),
        exp: (Utc::now() + Duration::hours(1)).timestamp(),
    };

    // Encode the JWT
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set in .env");
    let encoding_key = EncodingKey::from_secret(secret_key.as_bytes());

    // Generate the JWT token
    let token = encode(&Header::default(), &claims, &encoding_key)?;

    Ok(token)
}
