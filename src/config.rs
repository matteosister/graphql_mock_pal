//use graphql_mock_pal::errors;
//use graphql_mock_pal::errors::Result;
use graphql_mock_pal::matcher::Matcher;
//use snafu::ResultExt;
//use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
pub struct Config {
    matchers: Vec<Matcher>,
}

pub fn start(tx: Sender<Config>, receiver: Receiver<String>) {
    for msg in receiver {
        println!("config received {}", msg);
        let config = get_config();
        tx.send(config);
    }
}

fn get_config() -> Config {
    Config { matchers: vec![] }
}

/*fn parse_file(path: &str) -> Result<()> {
    let filename = Path::new(path);
    std::fs::read(filename).context(errors::OpenConfig {
        filename: filename.to_string_lossy().to_string(),
    })?;
    Ok(())
}*/
