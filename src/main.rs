use crate::args::Cli;
use crate::commands::{decode, encode, print, remove};
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Cli::parse();

    match args {
        Cli::Encode(args) => encode(args),
        Cli::Decode(args) => decode(args),
        Cli::Remove(args) => remove(args),
        Cli::Print(args) => print(args),
    }?;
    Ok(())
}
