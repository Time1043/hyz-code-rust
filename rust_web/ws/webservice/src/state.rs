// use super::models::Course;
use sqlx::postgres::PgPool;
use std::sync::Mutex;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    // pub courses: Mutex<Vec<Course>>,
    pub db: PgPool,  // 不用内存 用数据库存储 (连接池 多线程共享)
}
