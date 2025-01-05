use std::fmt;

#[derive(Debug)]
pub enum Error {
    /// Authentication related errors
    Auth(String),
    /// Token related errors
    Token(String),
    /// Network/HTTP errors from reqwest
    Request(ureq::Error),
    /// Errors from environment variables
    Environment(String),
    /// API response errors
    Api(String),
    // IO errors
    IO(std::io::Error),
    // postgres errors
    Postgres(postgres::Error),
    // csv errors
    Csv(csv::Error),
    // serde errors
    Serde(String),
    // Internal errors
    Internal(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Auth(msg) => write!(f, "Authentication error: {}", msg),
            Error::Token(msg) => write!(f, "Token error: {}", msg),
            Error::Request(e) => write!(f, "Request error: {}", e),
            Error::Environment(msg) => write!(f, "Environment error: {}", msg),
            Error::Api(msg) => write!(f, "API error: {}", msg),
            Error::IO(e) => write!(f, "IO error: {}", e),
            Error::Postgres(e) => write!(f, "Postgres error: {}", e),
            Error::Csv(e) => write!(f, "Csv error: {}", e),
            Error::Serde(msg) => write!(f, "Serde error: {}", msg),
            Error::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl From<ureq::Error> for Error {
    fn from(err: ureq::Error) -> Self {
        Error::Request(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<postgres::Error> for Error {
    fn from(err: postgres::Error) -> Self {
        Error::Postgres(err)
    }
}

impl From<csv::Error> for Error {
    fn from(err: csv::Error) -> Self {
        Error::Csv(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serde(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
