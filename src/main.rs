use base64::prelude::*;
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
        .and_then(|alg| Some(alg.to_lowercase()))
        .or(Some("argon2id".to_string()))
        .unwrap();

    let content = if args.base64 {
        Some(BASE64_STANDARD.decode(args.content.as_bytes())
            .or(Err(Error::new(ErrorKind::Decoding, "Invalid Base64 string")))?)
    } else {
        None
    };

    let phc = hash_content(
        content.as_ref()
            .and_then(|content| Some(content.as_slice()))
            .or(Some(args.content.as_bytes()))
            .unwrap(),
        alg.as_str(),
        args.salt.as_ref().map(|s| s.as_str())
    )?;

    println!("{}", phc);

    Ok(())
}
