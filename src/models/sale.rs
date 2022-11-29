use chrono::{NaiveDate, Local};
use serde::{Serialize, Deserialize};
use actix_web::{get, web, Result, Responder, post, HttpResponse};

use crate::models::helpers::make_connection_pool;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Sale {
    pub id: Option<i32>,
    pub timestamp: Option<NaiveDate>,
    pub menu_items_id: Vec<i32>,
    pub total_sales_price: f32,
    pub employee_id: i32,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct ItemSale {
    pub name: String,
    pub count: Option<i64>,
    pub sum: Option<f32>,
}

#[get("/api/sales")]
async fn get_sales() -> Result<impl Responder> {
    let pool = make_connection_pool().await;
    let rows: Vec<Sale> = sqlx::query_as("SELECT * FROM sales ORDER BY timestamp ASC").fetch_all(&pool).await.expect("Failed to execute query.");
    Ok(web::Json(rows))
}

#[post("/api/sales")]
async fn post_sales(data: web::Json<Sale>) -> HttpResponse {
    let time = Local::now().date_naive();
    let pool = make_connection_pool().await;
    match sqlx::query!("INSERT INTO sales (timestamp, menu_items_id, total_sales_price, employee_id) VALUES ($1, $2, $3, $4)",
        time, &data.menu_items_id, data.total_sales_price, data.employee_id)
        .execute(&pool)
        .await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => {
                println!("{}", e);
                HttpResponse::BadRequest().finish()
            }
        }
}

#[get("/api/sales/item/{from}/{to}")]
async fn get_sales_by_item(path: web::Path<(NaiveDate, NaiveDate)>) -> Result<impl Responder> {
    let (from, to) = path.into_inner();
    let pool = make_connection_pool().await;
    let rows: Vec<ItemSale> = sqlx::query_as!(ItemSale, "SELECT menu_items.name, COUNT(sales.id), SUM(menu_items.price)
        FROM menu_items, sales
        WHERE menu_items.id = ANY (sales.menu_items_id) AND sales.timestamp BETWEEN $1 AND $2
        GROUP BY menu_items.name",
        from, to)
        .fetch_all(&pool).await.expect("Failed to execute query.");
    Ok(web::Json(rows))
}