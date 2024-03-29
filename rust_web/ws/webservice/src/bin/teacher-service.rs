use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[path = "../dbaccess/mod.rs"]
mod dbaccess;
#[path = "../errors.rs"]
mod errors;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../models/mod.rs"]
mod models;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;

use routers::*;
use state::AppState;
use crate::errors::MyError;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set."); // 读取数据库连接信息
    let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap(); // 创建数据库连接池

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        // courses: Mutex::new(vec![]),
        db: db_pool, // 数据库连接池
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                MyError::InvalidInput("Please provide valid Json input".to_string()).into()
            }))
            .configure(general_routes)
            .configure(course_routes)
            .configure(teacher_routes)
    };
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
