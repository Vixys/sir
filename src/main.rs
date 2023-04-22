use std::process::ExitCode;

use clap::Parser;

pub mod error;
pub mod input;

use input::SourceInput;

/// Concatenate FILE(s) to standard output.
///
/// rcat is a Rust implementation of cat(1)
#[derive(Parser)]
struct RCatArgs {
    /// File(s) to concatenante to standard output.
    file: Vec<std::path::PathBuf>,
}

fn main() -> ExitCode {
    env_logger::init();
    let args = RCatArgs::parse();
    let mut exit_code = ExitCode::SUCCESS;

    for path in args.file {
        let source = input::FileInput::new(path);
        if let Err(err) = source.display(&mut std::io::stdout()) {
            eprintln!("{}", err);
            exit_code = ExitCode::FAILURE;
        }
    }

    exit_code
}
