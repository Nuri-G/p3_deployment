use chrono::{NaiveDate, Local};
use bigdecimal::BigDecimal;
use serde::{Serialize, Deserialize};
use actix_web::{get, web, Result, Responder, post, HttpResponse, put};

use crate::models::helpers::make_connection_pool;

/// Representation of Ingredients in the database.
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Ingredient {
    pub id: Option<i32>,
    pub item_name: String,
    pub item_amount: i32,
    pub storage_location: String,
    pub min_req: Option<i32>,
}

/// Representation of Excess for building an Excess report.
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Excess {
    pub ingredient: String,
    pub percent: Option<BigDecimal>,
}

/// Representation of a Restock for building the Restock report.
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Restock {
    pub item_name: String,
    pub item_amount: i32,
    pub min_req: Option<i32>,
}

/// Returns a JSON array of all the ingredients in the database.
#[get("/api/ingredients")]
pub async fn get_ingredients() -> Result<impl Responder> {
    let pool = make_connection_pool().await;
    let rows: Vec<Ingredient> = sqlx::query_as("SELECT * FROM ingredients_inventory ORDER BY item_name ASC").fetch_all(&pool).await.expect("Failed to execute query.");
    Ok(web::Json(rows))
}

/// Inserts an Ingredient into the database.
#[post("/api/ingredients")]
pub async fn post_ingredients(data: web::Json<Ingredient>) -> HttpResponse {
    let pool = make_connection_pool().await;
    match sqlx::query!("INSERT INTO ingredients_inventory (item_name, item_amount, storage_location, min_req) VALUES ($1, $2, $3, $4)",
        data.item_name, data.item_amount, data.storage_location, data.min_req)
        .execute(&pool)
        .await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::BadRequest().finish(),
        }
}

/// Updates an Ingredient in the database.
#[put("/api/ingredients")]
pub async fn put_ingredients(data: web::Json<Ingredient>) -> HttpResponse {
    let pool = make_connection_pool().await;
    match sqlx::query!("UPDATE ingredients_inventory SET item_name = $1, item_amount = $2, storage_location = $3, min_req = $4 WHERE id = $5",
        data.item_name, data.item_amount, data.storage_location, data.min_req, data.id)
        .execute(&pool)
        .await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::BadRequest().finish(),
        }
}

#[get("/api/ingredients/excess/{from}")]
pub async fn get_excess(path: web::Path<NaiveDate>) -> Result<impl Responder> {
    let from = path.into_inner();
    let now = Local::now().date_naive();
    let pool = make_connection_pool().await;
    let rows: Vec<Excess> = sqlx::query_as!(Excess, "WITH ingredientPercents (ingredient, percent) AS 
        (WITH menuItemsCount (ingredients, count) AS
        (SELECT menu_items.ingredients_inventory_id, COUNT(sales.id)
        FROM menu_items, sales
        WHERE menu_items.id = ANY (sales.menu_items_id)
        AND sales.timestamp >= $1 AND sales.timestamp <= $2
        GROUP BY menu_items.ingredients_inventory_id)
        SELECT ingredients_inventory.item_name, (SUM(menuItemsCount.count) / (SUM(menuItemsCount.count) + ingredients_inventory.item_amount)) * 100 AS percent
        FROM ingredients_inventory, menuItemsCount
        WHERE ingredients_inventory.id = ANY (menuItemsCount.ingredients)
        GROUP BY ingredients_inventory.item_name, ingredients_inventory.item_amount)
        SELECT ingredientPercents.ingredient, ROUND(ingredientPercents.percent, 2) as percent 
        FROM ingredientPercents WHERE percent < 10",
        from, now)
        .fetch_all(&pool).await.expect("Failed to execute query.");
    Ok(web::Json(rows))
}

#[get("/api/ingredients/restock")]
pub async fn get_restock() -> Result<impl Responder> {
    let pool = make_connection_pool().await;
    let rows: Vec<Restock> = sqlx::query_as("SELECT item_name, item_amount, min_req FROM ingredients_inventory WHERE item_amount < min_req")
        .fetch_all(&pool).await.expect("Failed to execute query.");
    Ok(web::Json(rows))
}