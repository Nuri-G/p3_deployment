use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct MenuItem {
    id: i32,
    name: String,
    category: String,
    price: f32,
}

impl MenuItem {
    pub fn new(id: i32, name: String, category: String, price: f32) -> Self {
        Self {
            id,
            name,
            category,
            price,
        }
    }
}