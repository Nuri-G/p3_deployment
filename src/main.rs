mod models;

use actix_cors::Cors;
use actix_web::{HttpServer, App};
use dotenvy::dotenv;

use crate::models::menu_item::{get_menu, post_menu};
use crate::models::sale::{get_sales, post_sales};
use crate::models::ingredients::{get_ingredients, post_ingredients, put_ingredients};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
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
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}