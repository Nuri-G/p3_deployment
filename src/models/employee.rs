use core::fmt;
use std::{pin::Pin, future::Future};

use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use actix_web::{get, web, Result, Responder, post, HttpResponse, put, FromRequest, ResponseError};
use serde_json::Value;

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

#[post("/api/auth")]
pub async fn user_from_token(e: Employee) -> Result<impl Responder> {
    Ok(web::Json(e))
}

#[derive(Debug)]
pub struct FromRequestError {
    status: StatusCode,
}

impl fmt::Display for FromRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for FromRequestError {
    fn status_code(&self) -> reqwest::StatusCode {
        self.status
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status).finish()
    }
}

impl FromRequest for Employee {
    type Error = FromRequestError;

    type Future = Pin<Box<dyn Future<Output = Result<Employee, FromRequestError>>>>;

    fn from_request(req: &actix_web::HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let head = req.head();
        let token = head.headers.get("token").unwrap().to_str().unwrap();
        let url = format!("https://oauth2.googleapis.com/tokeninfo?id_token={}", token);
        let client = reqwest::Client::new();
        Box::pin(async move {
            let res = client.get(url).send().await.unwrap().text().await.unwrap();
            println!("{}", res);
            let res_json: Value = serde_json::from_str(res.as_str()).unwrap();

            match &res_json["email"] {
                Value::String(email) => {
                    let pool = make_connection_pool().await;
                    let query = format!("SELECT * FROM employees WHERE employee_email = '{}';", email);
                    let mut rows: Vec<Employee> = sqlx::query_as(query.as_str()).fetch_all(&pool).await.expect("Failed to execute query.");
                    if rows.len() == 0 {
                        return Err(FromRequestError {status: StatusCode::UNAUTHORIZED});
                    }
                    let employee = rows.remove(0);
                    Ok(employee)
                },
                _ => Err(FromRequestError {status: StatusCode::UNAUTHORIZED}),
            }
        })
    }

    fn extract(req: &actix_web::HttpRequest) -> Self::Future {
        Self::from_request(req, &mut actix_web::dev::Payload::None)
    }
}