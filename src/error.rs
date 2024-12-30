use std::fmt;

#[derive(Debug)]
pub enum Error {
    /// Authentication related errors
    Auth(String),
    /// Token related errors 
    Token(String),
    /// Network/HTTP errors from reqwest
    Request(reqwest::Error),
    /// Errors from environment variables
    Environment(String),
    /// API response errors
    Api(String),
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
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Request(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
