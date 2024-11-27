use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum Error {
    Decoding(base64ct::Error),
    HashGen(String),
    SaltDecoding(password_hash::Error),
    Io(std::io::Error),
}

impl Error {
    pub fn hash_err(msg: Option<&str>) -> Self {
        Self::HashGen(msg.unwrap_or("Error occurred during hashing").to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HashGen(msg) => Display::fmt(msg, f),
            Self::Decoding(err) => Display::fmt(err, f),
            Self::SaltDecoding(err) => Display::fmt(err, f),
            Self::Io(err) => Display::fmt(err, f),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<base64ct::Error> for Error {
    fn from(value: base64ct::Error) -> Self {
        Self::Decoding(value)
    }
}

impl From<password_hash::Error> for Error {
    fn from(value: password_hash::Error) -> Self {
        Self::SaltDecoding(value)
    }
}
