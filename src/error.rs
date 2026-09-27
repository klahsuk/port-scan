use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Usage: port-scanner <target.com>")]
    CliUsage,
    #[error("Reqwest: {0}")]
    Reqwest(String),
}

//errors received from reqwest can now be converted into our error type
impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err.to_string())
    }
}