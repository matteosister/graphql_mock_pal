use actix_web::{web, App, HttpServer};

pub mod controller;
pub mod matcher;

use crate::controller::query_handler;
use actix_web::web::post;

fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::resource("/").route(post().to(query_handler))))
        .bind("127.0.0.1:8080")?
        .run()
}
