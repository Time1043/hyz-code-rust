#[path = "../mod.rs"]
mod wa;

use wa::{errors, handlers, models, routers};
use routers::app_config;
use std::env;
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use tera::Tera;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host_port = env::var("HOST_PORT").expect("HOST:PORT address is not exist");
    println!("Listening on: {}", &host_port);

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();
        App::new().app_data(web::Data::new(tera)).configure(app_config)
    }).bind(&host_port)?.run().await
}