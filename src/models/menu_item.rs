use std::pin::Pin;

use serde::{Serialize, Deserialize};
use actix_web::{get, web::{self, Json, Path}, Result, Responder, post, HttpResponse, put};

use crate::models::{helpers::make_connection_pool, translate::translate};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct MenuItem {
    pub id: Option<i32>,
    pub name: String,
    pub category: String,
    pub ingredients_inventory_id: Vec<i32>,
    pub price: f32,
    pub description: String,
}

async fn translate_menu_item(menu_item: &MenuItem, target_language: &str) -> MenuItem {
    const FROM: &str = "en";

    let name = translate(menu_item.name.to_owned(), FROM, target_language);
    let category = translate(menu_item.category.to_owned(), FROM, target_language);
    let description = translate(menu_item.description.to_owned(), FROM, target_language);
    MenuItem {
        id: menu_item.id,
        name: name.await,
        category: category.await,
        ingredients_inventory_id: menu_item.ingredients_inventory_id.clone(),
        price: menu_item.price,
        description: description.await,
    }
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

#[get("/api/menu/{language}")]
pub async fn get_menu_translated(language: Path<String>) -> Result<impl Responder> {
    let language = language.into_inner();
    let mut json = get_menu_items().await.unwrap();

    if language == "en" {
        return Ok(json);
    }

    let mut futures= Vec::<Pin<Box<dyn std::future::Future<Output = MenuItem>>>>::new();
    for item in json.iter_mut() {
        futures.push(Box::pin(translate_menu_item(item, &language)));
    }
    let mut output = vec![];
    for future in futures.iter_mut() {
        output.push(future.await);
    }
    
    Ok(Json(output))
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