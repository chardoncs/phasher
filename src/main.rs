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
        .and_then(|alg| Some(alg.to_lowercase()))
        .or(Some("argon2id".to_string()))
        .unwrap();

    println!("{}", hash_content(args.content.as_str(), alg.as_str(), args.salt.as_ref().map(|s| s.as_str()))?);

    Ok(())
}
