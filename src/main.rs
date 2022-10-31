use std::fs;

use actix_web::{get, web, App, Result, HttpServer, Responder, post, HttpResponse, HttpResponseBuilder};
mod models;
use models::menu_item::{MenuItem, self};
use sqlx::{postgres::PgPoolOptions, Row, Postgres, Pool, Error};
mod secrets;

async fn make_connection_pool() -> Pool<Postgres> {
    let connection_string = format!("postgres://{}:{}@{}/{}", secrets::USERNAME, secrets::PASSWORD, secrets::URL, secrets::DB_NAME);
    PgPoolOptions::new()
        .max_connections(5)
        .connect(connection_string.as_str())
        .await.expect("Failed to make connection to database.")
}

#[get("/api/menu")]
async fn get_menu() -> Result<impl Responder> {
    let pool = make_connection_pool().await;
    let rows = sqlx::query("SELECT id, name, category, price FROM menu_items").fetch_all(&pool).await.expect("Failed to execute query.");

    let mut items = Vec::<MenuItem>::new();
    for row in rows {
        let item = MenuItem::new(row.get(0), row.get(1), row.get(2), row.get(3));
        items.push(item);
    }

    Ok(web::Json(items))
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_menu)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}