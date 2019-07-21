pub mod application;
pub mod cli;
pub mod config;
pub mod errors;
pub mod matcher;
pub mod state;

use crate::application::query_handler;
use crate::config::start;
use crate::state::AppState;
use actix_web::{web, web::post, App, HttpServer};
use cli::app;
use snafu::Snafu;
use std::sync::mpsc;
use std::thread;

fn main() -> std::io::Result<()> {
    let matches = app().get_matches();
    let (main_tx, main_rx) = mpsc::channel();
    let (config_tx, config_rx) = mpsc::channel();

    thread::spawn(move || start(main_tx, config_rx));

    config_tx.send("dammi la config".to_owned()).unwrap();
    let config = main_rx.recv().unwrap();
    println!("{:?}", config);

    config_tx.send("dammi la config".to_owned()).unwrap();
    let config = main_rx.recv().unwrap();
    println!("{:?}", config);

    HttpServer::new(|| {
        App::new()
            .data(AppState::new(vec![]))
            .service(web::resource("/graphql").route(post().to(query_handler)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not open config file: {}", source))]
    OpenConfig { source: std::io::Error },
}
