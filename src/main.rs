use base64ct::{Base64, Encoding};
use clap::Parser;
use cli::Cli;
use errors::{Error, ErrorKind};
use hashing::hash_content;

mod cli;
mod errors;
mod hashing;

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let alg = args.alg
        .map(|alg| alg.to_lowercase())
        .unwrap_or("argon2id".to_string());

    let encoded_content = if args.base64 {
        Some(Base64::decode_vec(args.content.as_str())
            .or(Err(Error::new(ErrorKind::Decoding, "Invalid Base64 string")))?)
    } else {
        None
    };

    let salt = args.salt;

    let phc = hash_content(
        encoded_content.as_ref()
            .map(|content| content.as_slice())
            .unwrap_or(args.content.as_bytes()),
        alg.as_str(),
        salt.as_ref().map(|s| s.as_str()),
    )?;

    println!("{}", phc);

    Ok(())
}
