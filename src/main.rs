mod models {
    pub mod product_model;
    pub mod roles;
    pub mod user;
}

mod api {
    pub mod product_api;
    pub mod roles_api;
    pub mod users_api;
}

mod middleware {
    pub mod admin_middleware;
    pub mod auth_middleware;
}

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use dotenv::dotenv;

use api::roles_api::{create_admin_role, create_role};
use api::users_api::{get_data, login, register};

use middleware::admin_middleware::AdminMiddleware;
use middleware::auth_middleware::AuthMiddleWare;

use models::product_model::Product;
use models::roles::Roles;
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

    println!("Connected to MongoDB database");

    let user_collections: Collection<User> = db.collection("users");
    let product_collections: Collection<Product> = db.collection("products");
    let roles_collections: Collection<Roles> = db.collection("roles");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(user_collections.clone()))
            .app_data(Data::new(product_collections.clone()))
            .app_data(Data::new(roles_collections.clone()))
            .service(login)
            .service(register)
            .service(create_admin_role)
            .service(
                web::resource("/api/roles/create").route(
                    web::post()
                        .to(create_role)
                        .wrap(AdminMiddleware {
                            db: Data::new(user_collections.clone()),
                        })
                        .wrap(AuthMiddleWare),
                ),
            )
            .service(
                web::resource("/api/users/me")
                    .route(web::get().to(get_data))
                    .wrap(AdminMiddleware {
                        db: Data::new(user_collections.clone()),
                    })
                    .wrap(AuthMiddleWare),
            )
    })
    .bind("127.0.0.1:8000")?;

    println!("Server is up and running at http://127.0.0.1:8000");

    server.run().await
}
