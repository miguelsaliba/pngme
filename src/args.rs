use clap::{Args, Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub enum Cli {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Debug, Args)]
pub struct EncodeArgs {
    pub path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct DecodeArgs {
    pub path: PathBuf,
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    pub path: PathBuf,
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct PrintArgs {
    pub path: PathBuf,
}
