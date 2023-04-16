use std::{fs, path::Path};

use clap::{Parser, Subcommand};
use duct::cmd;

type DynError = Box<dyn std::error::Error>;

const COVERAGE_FOLDER: &str = "coverage";

#[derive(Parser)]
struct XtaskArgs {
    /// Run test and generate coverage report
    #[command(subcommand)]
    command: Option<XtaskCommands>,
}

enum CoverageFormat {
    Html,
    Markdown,
    // Lcov
}
#[derive(Subcommand)]
enum XtaskCommands {
    Coverage,
}

fn main() {
    let xtask = XtaskArgs::parse();

    match xtask.command {
        Some(XtaskCommands::Coverage) => cover().unwrap(),
        None => {}
    }
}

fn cover() -> Result<(), DynError> {
    if Path::new(COVERAGE_FOLDER).try_exists().unwrap() {
        fs::remove_dir_all(COVERAGE_FOLDER)?;
    }
    fs::create_dir_all(COVERAGE_FOLDER)?;

    println!("=== running coverage ===");

    cmd!("cargo", "test")
        .env("CARGO_INCREMENTAL", "0")
        .env("RUSTFLAGS", "-Cinstrument-coverage")
        .env("LLVM_PROFILE_FILE", "cargo-test-%p-%m.profraw")
        .run()?;

    println!("Ok");

    println!("=== generating report ===");
    let (fmt, file) = match CoverageFormat::Markdown {
        CoverageFormat::Html => ("html", "coverage/html"),
        CoverageFormat::Markdown => ("markdown", "coverage/html"),
        // CoverageFormat::Lcov => ("lcov", "coverage/tests.lcov")
    };
    cmd!(
        "grcov",
        ".",
        "--binary-path",
        "./target/debug/deps",
        "-s",
        ".",
        "-t",
        fmt,
        "--branch",
        "--ignore-not-existing",
        "--ignore",
        "../*",
        "--ignore",
        "/*",
        "--ignore",
        "xtask/*",
        "--ignore",
        "*/src/tests/*",
        // "-o",
        // file,
    )
    .run()?;

    println!("Ok");

    println!("report location: {file}");

    cmd!("find", ".", "-name", "*.profraw", "-exec", "rm", "{}", ";").run()?;

    Ok(())
}
