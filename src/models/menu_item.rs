use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct MenuItem {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub ingredients_inventory_id: Vec<i32>,
    pub price: f32,
}