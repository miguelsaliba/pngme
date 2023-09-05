use clap::Parser;

use crate::args::Command;

// use crate::args::Cli;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Command::parse();

    println!("{:?}", args);

    Ok(())
}
