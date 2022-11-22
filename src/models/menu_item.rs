use serde::{Serialize, Deserialize};
use actix_web::{get, web::{self, Path, Json}, Result, Responder, post, HttpResponse, put};

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
    let json = get_menu_items().await.unwrap();

    if language == "en" {
        let json_string = serde_json::to_string(&json).unwrap();
        return  Ok(json_string);
    }

    let mut translated = "{".to_string();
    for a in json.iter() {
        let s = serde_json::to_string(a).unwrap();
        let tr = translate(s, "en".to_string(), language.clone()).await.split("\n").next().unwrap().to_owned();
        translated += &tr;
        translated += ",";
    }
    translated = translated[0.. translated.len() - 1].to_string();
    translated += "}";
    Ok(translated)
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