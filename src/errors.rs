use std::fmt::Display;

#[derive(Debug)]
pub enum ErrorKind {
    HashGen,
    SaltDecoding,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::SaltDecoding => "Salt decoding",
            Self::HashGen => "Hash generation",
        })
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    msg: String,
}

impl Error {
    pub fn new(kind: ErrorKind, msg: &str) -> Self {
        Self {
            kind,
            msg: msg.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[error] {}: {}", self.kind, self.msg)
    }
}
