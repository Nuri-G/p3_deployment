mod models;

use std::env;

use actix_cors::Cors;
use actix_web::{HttpServer, App};
use dotenvy::dotenv;

use crate::models::menu_item::{get_menu, post_menu};
use crate::models::sale::{get_sales, post_sales};
use crate::models::ingredients::{get_ingredients, post_ingredients, put_ingredients};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host = env::var("HOST").expect("Failed to read 'HOST' environment variable.
        Use 127.0.0.1 for local or 0.0.0.0 for deployment.");
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(get_menu)
            .service(post_menu)
            .service(get_sales)
            .service(post_sales)
            .service(get_ingredients)
            .service(post_ingredients)
            .service(put_ingredients)
    })
    .bind((host, 80))?
    .run()
    .await
}