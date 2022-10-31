use actix_web::{get, web, App, Result, HttpServer, Responder, post, HttpResponse};
mod models;
use models::menu_item::{MenuItem};
use sqlx::{postgres::PgPoolOptions, Postgres, Pool};
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
    let rows: Vec<MenuItem> = sqlx::query_as("SELECT * FROM menu_items").fetch_all(&pool).await.expect("Failed to execute query.");
    Ok(web::Json(rows))
}

#[post("/api/menu")]
async fn post_menu(data: web::Json<MenuItem>) -> HttpResponse {
    let pool = make_connection_pool().await;
    match sqlx::query!("INSERT INTO menu_items (id, name, ingredients_inventory_id, category, price) VALUES ($1, $2, $3, $4, $5)",
        data.id, data.name, &data.ingredients_inventory_id, data.category, data.price)
        .execute(&pool)
        .await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::BadRequest().finish(),
        }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(get_menu)
            .service(post_menu)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}