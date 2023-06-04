mod models {
    pub mod user;
}

mod api {
    pub mod users_api;
}

use actix_web::{web, App, HttpServer};
use api::users_api::{get_data, login, register};
use dotenv::dotenv;
use log::info;
use models::user::User;
use mongodb::{options::ClientOptions, Client, Collection};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();
    let mongodb_url = env::var("MONGODB_URL").expect("MONGODB_URL must be set in .env");
    env::var("SECRET_KEY").expect("SECRET_KEY must be set in .env");

    let client_options = ClientOptions::parse(mongodb_url)
        .await
        .expect("Failed to parse MongoDB options");

    let client = Client::with_options(client_options).expect("Failed to create MongoDB client");
    let db = client.database("rust");

    info!("Connected to MongoDB database");

    let user_collections: Collection<User> = db.collection("users");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_collections.clone()))
            .service(get_data)
            .service(login)
            .service(register)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
