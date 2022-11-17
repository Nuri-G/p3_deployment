use serde::{Serialize, Deserialize};
use actix_web::{get, web, Result, Responder, post, HttpResponse, put};

use crate::models::helpers::make_connection_pool;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Employee {
    pub id: Option<i32>,
    pub employee_name: String,
    pub employee_email: String,
    pub employee_phone: String,
    pub job_title: String,
    pub is_manager: bool,
    pub hourly_salary: f32
}

#[get("/api/employees")]
pub async fn get_employees() -> Result<impl Responder> {
    let pool = make_connection_pool().await;
    let rows: Vec<Employee> = sqlx::query_as("SELECT * FROM employees ORDER BY employee_name ASC").fetch_all(&pool).await.expect("Failed to execute query.");
    Ok(web::Json(rows))
}

#[post("/api/employees")]
pub async fn post_employees(data: web::Json<Employee>) -> HttpResponse {
    let pool = make_connection_pool().await;
    match sqlx::query!("INSERT INTO employees (employee_name, employee_email, employee_phone, job_title, is_manager, hourly_salary) VALUES ($1, $2, $3, $4, $5, $6)",
        data.employee_name, data.employee_email, data.employee_phone, data.job_title, data.is_manager, data.hourly_salary)
        .execute(&pool)
        .await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::BadRequest().finish(),
        }
}

#[put("/api/employees")]
pub async fn put_employees(data: web::Json<Employee>) -> HttpResponse {
    let pool = make_connection_pool().await;
    match sqlx::query!("UPDATE employees SET employee_name = $1, employee_email = $2, employee_phone = $3, job_title = $4, is_manager = $5, hourly_salary = $6 WHERE id = $7",
    data.employee_name, data.employee_email, data.employee_phone, data.job_title, data.is_manager, data.hourly_salary, data.id)
        .execute(&pool)
        .await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::BadRequest().finish(),
        }
}