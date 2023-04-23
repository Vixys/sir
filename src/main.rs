use std::process::ExitCode;

use clap::Parser;

pub mod error;
pub mod input;
mod modifiers;

use input::SourceInput;
use modifiers::{show_end::ShowEnd, ModifierCollection};

/// Concatenate FILE(s) to standard output.
///
/// rcat is a Rust implementation of cat(1)
#[derive(Parser)]
#[command(version)]
struct RCatArgs {
    /// File(s) to concatenante to standard output.
    file: Vec<std::path::PathBuf>,

    /// Display $ at the end of each line.
    #[arg(short = 'E', long)]
    show_ends: bool,
}

fn main() -> ExitCode {
    env_logger::init();
    let args = RCatArgs::parse();
    let mut exit_code = ExitCode::SUCCESS;
    let mut modifier_collection = ModifierCollection::new();

    if args.show_ends {
        modifier_collection.add(Box::new(ShowEnd::new()));
    }

    for path in args.file {
        let source = input::FileInput::new(path);
        if let Err(err) = source.display(&mut modifier_collection) {
            eprintln!("{}", err);
            exit_code = ExitCode::FAILURE;
        }
    }

    exit_code
}
