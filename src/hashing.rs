use std::str::FromStr;

use crate::errors::Error;
use argon2::{Algorithm, Argon2, Params, Version};
use password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
use pbkdf2::Pbkdf2;
use scrypt::Scrypt;

pub fn hash_content(bytes: &[u8], alg: &str, salt: Option<&str>) -> Result<String, Error> {
    let salt = if let Some(salt) = salt {
        SaltString::from_b64(salt)?
    } else {
        SaltString::generate(&mut OsRng)
    };

    match alg {
        "argon2i" | "argon2d" | "argon2id" => {
            let ctx = Argon2::new(
                Algorithm::from_str(alg).unwrap(),
                Version::default(),
                Params::default(),
            );

            ctx.hash_password(bytes, &salt)
                .or(Err(Error::hash_err(None)))
        }
        "scrypt" => {
            Scrypt.hash_password(bytes, &salt)
                .or(Err(Error::hash_err(None)))
        }
        "pbkdf2" => {
            Pbkdf2.hash_password(bytes, &salt)
                .or(Err(Error::hash_err(None)))
        }
        _ => {
            Err(Error::hash_err(Some("Unsupported algorithm")))
        }
    }.map(|result| result.to_string())
}
