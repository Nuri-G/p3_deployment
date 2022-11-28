mod models;

use std::env;

use actix_cors::Cors;
use actix_web::{HttpServer, App};
use dotenvy::dotenv;
use models::employee::user_from_token;
use models::menu_item::get_menu_translated;
use models::translate::translated_keywords;

use crate::models::menu_item::{get_menu, post_menu, put_menu};
use crate::models::sale::{get_sales, post_sales};
use crate::models::ingredients::{get_ingredients, post_ingredients, put_ingredients};
use crate::models::employee::{get_employees, post_employees, put_employees};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host = env::var("HOST").expect("Failed to read 'HOST' environment variable.
        Use 127.0.0.1 for local or 0.0.0.0 for deployment.");
    let port = env::var("PORT").unwrap().parse::<u16>().unwrap();
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(get_menu)
            .service(get_menu_translated)
            .service(post_menu)
            .service(put_menu)
            .service(get_sales)
            .service(post_sales)
            .service(get_ingredients)
            .service(post_ingredients)
            .service(put_ingredients)
            .service(get_employees)
            .service(post_employees)
            .service(put_employees)
            .service(user_from_token)
            .service(translated_keywords)
    })
    .bind((host, port))?
    .run()
    .await
}