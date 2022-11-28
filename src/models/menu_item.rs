use std::{future::Future, pin::Pin};

use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use actix_web::{get, web::{self, Json, Path}, Result, Responder, post, HttpResponse, put};

use crate::models::{helpers::make_connection_pool, translate::translate};

use super::translate::Translate;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct MenuItem {
    pub id: Option<i32>,
    pub name: String,
    pub category: String,
    pub ingredients_inventory_id: Vec<i32>,
    pub price: f32,
    pub description: String,
}

async fn get_menu_items() -> Result<Json<Vec<MenuItem>>> {
    let pool = make_connection_pool().await;
    let rows: Vec<MenuItem> = sqlx::query_as("SELECT * FROM menu_items ORDER BY name ASC").fetch_all(&pool).await.expect("Failed to execute query.");
    let json = web::Json(rows);

    return  Ok(json);
}

#[get("/api/menu")]
pub async fn get_menu() -> Result<impl Responder> {
    get_menu_items().await

}

#[async_trait]
impl Translate for MenuItem {
    async fn translate(&mut self, target_language: &str) {
        const FROM: &str = "en";

        let name = translate(self.name.to_owned(), FROM, target_language);
        let category = translate(self.category.to_owned(), FROM, target_language);
        let description = translate(self.description.to_owned(), FROM, target_language);
        self.name = name.await;
        self.category = category.await;
        self.description = description.await;
    }
}

#[get("/api/menu/{language}")]
pub async fn get_menu_translated(language: Path<String>) -> Result<impl Responder> {
    let language = language.into_inner();
    let mut json = get_menu_items().await.unwrap();

    if language == "en" {
        return Ok(json);
    }

    let mut futures= Vec::<Pin<Box<dyn std::future::Future<Output = ()> + Send>>>::new();
    let mut output: Vec<MenuItem> = vec![];
    for a in json.iter_mut() {
        futures.push(a.translate(&language));
        output.push(a);
    }
    
    Ok(json)
}

#[post("/api/menu")]
pub async fn post_menu(data: web::Json<MenuItem>) -> HttpResponse {
    let pool = make_connection_pool().await;
    match sqlx::query!("INSERT INTO menu_items (name, ingredients_inventory_id, category, price, description) VALUES ($1, $2, $3, $4, $5)",
        data.name, &data.ingredients_inventory_id, data.category, data.price, data.description)
        .execute(&pool)
        .await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::BadRequest().finish(),
        }
}

#[put("/api/menu")]
pub async fn put_menu(data: web::Json<MenuItem>) -> HttpResponse {
    let pool = make_connection_pool().await;
    match sqlx::query!("UPDATE menu_items SET name = $1, ingredients_inventory_id = $2, category = $3, price = $4, description = $5 WHERE id = $6",
        data.name, &data.ingredients_inventory_id, data.category, data.price, data.description, data.id)
        .execute(&pool)
        .await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::BadRequest().finish(),
        }
}