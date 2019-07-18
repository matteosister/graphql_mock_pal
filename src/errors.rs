use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not open config file {}: {}", filename, source))]
    #[snafu(visibility(pub))]
    OpenConfig {
        source: std::io::Error,
        filename: String,
    },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
