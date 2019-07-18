pub mod application;
pub mod cli;
pub mod errors;
pub mod matcher;
pub mod state;

use crate::application::query_handler;
use crate::errors::*;
use crate::state::AppState;
use actix_web::{web, web::post, App, HttpServer};
use cli::app;
use snafu::{ResultExt, Snafu};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let matches = app().get_matches();
    dbg!(matches.value_of("file"));

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

fn parse_file() -> Result<()> {
    let filename = Path::new("mocks.json");
    std::fs::read(filename).context(errors::OpenConfig {filename: filename.to_string_lossy().to_string()})?;
    Ok(())
}
