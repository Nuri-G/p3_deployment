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

#[get("/api/sales")]
async fn get_sales() -> Result<impl Responder> {
    let pool = make_connection_pool().await;
    let rows: Vec<Sale> = sqlx::query_as("SELECT * FROM sales").fetch_all(&pool).await.expect("Failed to execute query.");
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
                return HttpResponse::BadRequest().finish()
            }
        }
}