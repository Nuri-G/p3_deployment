mod models;

use actix_web::{HttpServer, App};
use dotenvy::dotenv;

use crate::models::menu_item::{get_menu, post_menu};
use crate::models::sale::{get_sales, post_sales};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(get_menu)
            .service(post_menu)
            .service(get_sales)
            .service(post_sales)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}