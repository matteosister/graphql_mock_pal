pub mod application;
pub mod matcher;

use crate::application::query_handler;
use actix_web::{web, web::post, App, HttpServer};
use {clap, clap::Arg};

fn main() -> std::io::Result<()> {
    let matches = clap::App::new("GraphQL Mock Pal CLI")
        .version("0.1")
        .author("Matteo G. <matteog@gmail.com>")
        .about("CLI for Http Mock Pal utility")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("A json file with the graphql matchers")
                .takes_value(true),
        )
        .get_matches();
    dbg!(matches.value_of("file"));

    HttpServer::new(|| {
        App::new().service(web::resource("/graphql").route(post().to(query_handler)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}
