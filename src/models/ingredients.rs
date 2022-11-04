use serde::{Serialize, Deserialize};
use actix_web::{get, web, Result, Responder, post, HttpResponse};

use crate::models::helpers::make_connection_pool;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Ingredient {
    pub id: i32,
    pub item_name: String,
    pub item_amount: i32,
    pub storage_location: String,
}

#[get("/api/ingredients")]
pub async fn get_ingredients() -> Result<impl Responder> {
    let pool = make_connection_pool().await;
    let rows: Vec<Ingredient> = sqlx::query_as("SELECT * FROM ingredients_inventory").fetch_all(&pool).await.expect("Failed to execute query.");
    Ok(web::Json(rows))
}

#[post("/api/ingredients")]
pub async fn post_ingredients(data: web::Json<Ingredient>) -> HttpResponse {
    let pool = make_connection_pool().await;
    match sqlx::query!("INSERT INTO ingredients_inventory (id, item_name, item_amount, storage_location) VALUES ($1, $2, $3, $4)",
        data.id, data.item_name, data.item_amount, data.storage_location)
        .execute(&pool)
        .await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::BadRequest().finish(),
        }
}