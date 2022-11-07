use serde::{Serialize, Deserialize};
use actix_web::{get, web, Result, Responder, post, HttpResponse};

use crate::models::helpers::make_connection_pool;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct MenuItem {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub ingredients_inventory_id: Vec<i32>,
    pub price: f32,
    pub description: String,
}

#[get("/api/menu")]
pub async fn get_menu() -> Result<impl Responder> {
    let pool = make_connection_pool().await;
    let rows: Vec<MenuItem> = sqlx::query_as("SELECT * FROM menu_items").fetch_all(&pool).await.expect("Failed to execute query.");
    Ok(web::Json(rows))
}

#[post("/api/menu")]
pub async fn post_menu(data: web::Json<MenuItem>) -> HttpResponse {
    let pool = make_connection_pool().await;
    match sqlx::query!("INSERT INTO menu_items (id, name, ingredients_inventory_id, category, price, description) VALUES ($1, $2, $3, $4, $5, $6)",
        data.id, data.name, &data.ingredients_inventory_id, data.category, data.price, data.description)
        .execute(&pool)
        .await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::BadRequest().finish(),
        }
}