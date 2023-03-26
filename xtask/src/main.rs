use clap::{Parser, Subcommand};

type DynError = Box<dyn std::error::Error>;

#[derive(Parser)]
struct XtaskArgs {
    #[command(subcommand)]
    command: Option<XtaskCommands>,
}

#[derive(Subcommand)]
enum XtaskCommands {
    Coverage ,
}

fn main() {
    let xtask = XtaskArgs::parse();

    match xtask.command {
        Some(XtaskCommands::Coverage) => cover().unwrap(),
        None => {}
    }
}

fn cover() -> Result<(), DynError> {
    println!("=== running coverage ===");

    Ok(())
}
