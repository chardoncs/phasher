use std::str::FromStr;

use crate::errors::{Error, ErrorKind};
use argon2::{Algorithm, Argon2, Params, Version};
use password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
use pbkdf2::Pbkdf2;
use scrypt::Scrypt;

pub fn hash_content(content: &str, alg: &str, salt: Option<&str>) -> Result<String, Error> {
    let salt = if let Some(salt) = salt {
        SaltString::from_b64(salt)
            .or(Err(Error::new(ErrorKind::SaltDecoding, "Invalid base64 salt")))?
    } else {
        SaltString::generate(&mut OsRng)
    };

    let bytes = content.as_bytes();

    match alg {
        "argon2i" | "argon2d" | "argon2id" => {
            let ctx = Argon2::new(Algorithm::from_str(alg).unwrap(), Version::default(), Params::default());

            ctx.hash_password(bytes, &salt)
                .or(Err(Error::new(ErrorKind::HashGen, "Error occurred during hashing")))
        }
        "scrypt" => {
            Scrypt.hash_password(bytes, &salt)
                .or(Err(Error::new(ErrorKind::HashGen, "Error occurred during hashing")))
        }
        "pbkdf2" => {
            Pbkdf2.hash_password(bytes, &salt)
                .or(Err(Error::new(ErrorKind::HashGen, "Error occurred during hashing")))
        }
        _ => {
            Err(Error::new(ErrorKind::HashGen, "Unsupported algorithm"))
        }
    }.and_then(|result| Ok(result.to_string()))
}
