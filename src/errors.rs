use std::fmt::{Debug, Display};

pub enum ErrorKind {
    //Decoding,
    HashGen,
    SaltDecoding,
    Io(std::io::Error),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buffer: String;
        
        write!(f, "{}", match self {
            //Self::Decoding => "Content decoding",
            Self::SaltDecoding => "Salt decoding",
            Self::HashGen => "Hash generation",
            Self::Io(inner) => {
                buffer = inner.to_string();
                buffer.as_str()
            },
        })
    }
}

impl Debug for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
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
        write!(f, "{} ({})", self.kind, self.msg)
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::new(ErrorKind::Io(value), "")
    }
}
