use argon2::{Argon2, password_hash::PasswordHasher};
use base64::prelude::*;
use clap::Parser;
use cli::Cli;
use errors::{Error, ErrorKind};

mod cli;
mod errors;

fn decode_salt(salt: &str, decode_alg: &str) -> Result<Vec<u8>, Error> {
    Ok(match decode_alg {
        "base32" => {
            base32::decode(base32::Alphabet::Rfc4648 { padding: true }, salt)
                .ok_or(Error::new(ErrorKind::SaltDecoding, "Invalid base32 string"))?
        }
        "base32hex" => {
            base32::decode(base32::Alphabet::Rfc4648Hex { padding: true }, salt)
                .ok_or(Error::new(ErrorKind::SaltDecoding, "Invalid base32 string"))?
        }
        "base64" => {
            BASE64_STANDARD.decode(salt.as_bytes())
                .or(Err(Error::new(ErrorKind::SaltDecoding, "Invalid base64 string")))?
        }
        _ => {
            Err(Error::new(ErrorKind::SaltDecoding, "Unsupported algorithm"))?
        }
    })
}

fn hash_content(content: &str, alg: &str, salt: Option<&[u8]>) -> Result<String, Error> {
    match alg {
        _ => {
            Err(Error::new(ErrorKind::HashGen, "Unsupported algorithm"))?
        }
    }
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let alg = args.alg
        .and_then(|alg| Some(alg.to_lowercase()))
        .or(Some("argon2id".to_string()))
        .unwrap();

    let decoded_salt = if let Some(ref salt) = args.salt {
        if let Some(decode_alg) = args.decode_salt {
            Some(decode_salt(salt.as_str(), decode_alg.to_lowercase().as_str())?)
        } else {
            None
        }
    } else {
        None
    };

    let salt = args.salt
        .as_ref()
        .and_then(|salt| Some(if let Some(ref decoded_salt) = decoded_salt {
            decoded_salt.as_slice()
        } else {
            salt.as_bytes()
        }));

    println!("{}", hash_content(args.content.as_str(), alg.as_str(), salt)?);

    Ok(())
}
