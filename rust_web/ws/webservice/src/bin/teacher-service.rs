use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;

// 数据库连接
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[path = "../db_access.rs"]
mod db_access;
#[path = "../errors.rs"]
mod errors;
#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;

use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let databse_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set."); // 读取数据库连接信息
    let db_pool = PgPoolOptions::new().connect(&databse_url).await.unwrap(); // 创建数据库连接池

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        // courses: Mutex::new(vec![]),
        db: db_pool, // 数据库连接池
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
