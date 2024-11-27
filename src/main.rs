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
        let mut list = Vec::new();

        for line_ref in input_lines.iter() {
            list.push(Base64::decode_vec(line_ref.as_str())?);
        }

        owned_content_list = Some(list);
        owned_content_list.as_ref()
            .unwrap()
            .iter()
            .map(|vector| vector.as_slice())
            .collect()
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
