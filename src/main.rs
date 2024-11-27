use std::io::BufRead;

use base64ct::{Base64, Encoding};
use clap::Parser;
use cli::Cli;
use errors::Error;
use hashing::hash_content;

mod cli;
mod errors;
mod hashing;

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let alg = args.alg
        .map(|alg| alg.to_lowercase())
        .unwrap_or("argon2id".to_string());

    let input_lines = if args.content.is_empty() {
        let mut stdin_vec = Vec::new();

        for line in std::io::stdin().lock().lines() {
            stdin_vec.push(line?);
        }

        stdin_vec
    } else {
        args.content
    };

    let owned_content_list: Option<Vec<Vec<u8>>>;
    let content_list: Vec<&[u8]> = if args.base64 {
        owned_content_list = Some(
            input_lines.iter()
                .map(|item|
                    Base64::decode_vec(item.as_str())
                        .expect("Error: Invalid Base64")
                )
                .collect()
        );

        owned_content_list.as_ref().unwrap().iter().map(|vector| vector.as_slice()).collect()
    } else {
        input_lines.iter().map(|item| item.as_bytes()).collect()
    };

    let salt = args.salt;

    for bytes_ref in content_list {
        let phc = hash_content(
            bytes_ref,
            alg.as_str(),
            salt.as_ref().map(|s| s.as_str()),
        )?;

        println!("{phc}");
    }


    Ok(())
}
