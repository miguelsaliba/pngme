use clap::Parser;
use std::path::PathBuf;

// #[derive(Debug, Parser)]
// pub struct Cli {
//     #[command(subcommand)]
//     command: Command,
// }

#[derive(Parser, Debug)]
pub enum Command {
    Encode {
        path: PathBuf,
        chunk_type: String,
        message: String,
        output: Option<PathBuf>,
    },
    Decode {
        path: PathBuf,
        chunk_type: String,
    },
    Remove {
        path: PathBuf,
        chunk_type: String,
    },
    Print {
        path: PathBuf,
    },
}
