use serde::{Serialize, Deserialize};
use actix_web::{get, web, Result, Responder, post, HttpResponse, put};

use crate::models::helpers::make_connection_pool;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Ingredient {
    pub id: Option<i32>,
    pub item_name: String,
    pub item_amount: i32,
    pub storage_location: String,
}

#[get("/api/ingredients")]
pub async fn get_ingredients() -> Result<impl Responder> {
    let pool = make_connection_pool().await;
    let rows: Vec<Ingredient> = sqlx::query_as("SELECT * FROM ingredients_inventory ORDER BY item_name ASC").fetch_all(&pool).await.expect("Failed to execute query.");
    Ok(web::Json(rows))
}

#[post("/api/ingredients")]
pub async fn post_ingredients(data: web::Json<Ingredient>) -> HttpResponse {
    let pool = make_connection_pool().await;
    match sqlx::query!("INSERT INTO ingredients_inventory (item_name, item_amount, storage_location) VALUES ($1, $2, $3)",
        data.item_name, data.item_amount, data.storage_location)
        .execute(&pool)
        .await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::BadRequest().finish(),
        }
}

#[put("/api/ingredients")]
pub async fn put_ingredients(data: web::Json<Ingredient>) -> HttpResponse {
    let pool = make_connection_pool().await;
    match sqlx::query!("UPDATE ingredients_inventory SET item_amount = $1, storage_location = $2 WHERE id = $3",
        data.item_amount, data.storage_location, data.id)
        .execute(&pool)
        .await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::BadRequest().finish(),
        }
}