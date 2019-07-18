use clap::{App, Arg};

pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("GraphQL Mock Pal CLI")
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
}
