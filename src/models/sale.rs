use chrono::{NaiveDate};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Sale {
    pub id: i32,
    pub timestamp: NaiveDate,
    pub menu_items_id: Vec<i32>,
    pub total_sales_price: f32,
    pub employee_id: i32,
}