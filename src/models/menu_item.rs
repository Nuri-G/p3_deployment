use actix_rt::task::JoinHandle;
use serde::{Serialize, Deserialize};
use actix_web::{get, web::{self, Json, Path}, Result, Responder, post, HttpResponse, put, rt};

use crate::models::{helpers::make_connection_pool, translate::{translate, TranslationCache}};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct MenuItem {
    pub id: Option<i32>,
    pub name: String,
    pub category: String,
    pub ingredients_inventory_id: Vec<i32>,
    pub price: f32,
    pub description: String,
}

impl Clone for MenuItem {
    fn clone(&self) -> Self {
        Self { id: self.id.clone(), name: self.name.clone(), category: self.category.clone(), ingredients_inventory_id: self.ingredients_inventory_id.clone(), price: self.price.clone(), description: self.description.clone() }
    }
}

async fn translate_menu_item(menu_item: MenuItem, target_language: String) -> MenuItem {
    const FROM: &str = "en";

    let name = rt::spawn(translate(menu_item.name.to_owned(), FROM.to_owned(), target_language.to_owned()));
    let category = rt::spawn(translate(menu_item.category.to_owned(), FROM.to_owned(), target_language.to_owned()));
    let description = rt::spawn(translate(menu_item.description.to_owned(), FROM.to_owned(), target_language.to_owned()));
    MenuItem {
        id: menu_item.id,
        name: name.await.unwrap(),
        category: category.await.unwrap(),
        ingredients_inventory_id: menu_item.ingredients_inventory_id.clone(),
        price: menu_item.price,
        description: description.await.unwrap(),
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
pub async fn get_menu_translated(state: web::Data<TranslationCache>, language: Path<String>) -> Result<impl Responder> {
    let language = language.into_inner();
    let json = get_menu_items().await.unwrap();

    if language == "en" {
        return Ok(json);
    }

    let mut output = vec![];
    let mut futures= Vec::<(String, JoinHandle<MenuItem>)>::new();
    for item in json.iter() {
        let key = language.clone() + &serde_json::to_string(item).unwrap();
        match state.values.lock() {
            Ok(values) => {
                if values.contains_key(&key) {
                    let out = values.get(&key).unwrap().to_owned();
                    output.push(serde_json::from_str(&out).unwrap());
                } else {
                    futures.push((key.clone(), rt::spawn(translate_menu_item(item.clone(), language.clone()))));
                }
            },
            Err(_) => panic!("Translation mutex was poisoned."),
        }
        
    }
    
    while futures.len() > 0 {
        let future = futures.pop().unwrap();

        let key = &future.0;
        let result = future.1.await.unwrap();
        let result_str = serde_json::to_string(&result).unwrap();
        match state.values.lock() {
            Ok(mut values) => {
                values.insert(key.clone(), result_str.clone());
            },
            Err(_) => panic!("Translation mutex was poisoned."),
        }
        output.push(result);
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