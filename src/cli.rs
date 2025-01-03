use clap::Parser;

#[derive(Debug, Parser)]
#[command(about, version)]
pub struct Cli {
    #[arg(help = "Strings to be hashed. Reading from stdin if nothing is provided here")]
    pub content: Vec<String>,

    #[arg(short, long, help = "Flag the content as base64 encoded")]
    pub base64: bool,

    #[arg(short, long, help = "Hashing algorithm (case insensitive)\n - Options: Argon2i, Argon2d, Argon2id, scrypt, or PBKDF2\n - Argon2id by default")]
    pub alg: Option<String>,

    #[arg(short, long, help = "Salt, a base64 string by default, randomly generated if not specified")]
    pub salt: Option<String>,
}
