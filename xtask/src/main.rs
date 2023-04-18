use std::{fs, path::Path, fmt::Display};

use clap::{Parser, Subcommand, ValueEnum};
use duct::cmd;

type DynError = Box<dyn std::error::Error>;

const COVERAGE_FOLDER: &str = "coverage";

#[derive(Parser)]
struct XtaskArgs {
    /// Run test and generate coverage report
    #[command(subcommand)]
    command: Option<XtaskCommands>,
}

#[derive(Clone, Copy, ValueEnum)]
enum CoverageFormat {
    Html,
    Lcov,
    // Lcov
}

impl Display for CoverageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CoverageFormat::Html => "html",
            CoverageFormat::Lcov => "lcov",
        };
        write!(f, "{}", str)
    }
}

#[derive(Subcommand)]
enum XtaskCommands {
    Coverage { 
        #[arg(default_value_t = CoverageFormat::Lcov)]
        format: CoverageFormat
    },
}

fn main() {
    let xtask = XtaskArgs::parse();

    match xtask.command {
        Some(XtaskCommands::Coverage { format }) => cover(format).unwrap(),
        None => {}
    }
}

fn cover(format: CoverageFormat) -> Result<(), DynError> {
    if Path::new(COVERAGE_FOLDER).try_exists().unwrap() {
        fs::remove_dir_all(COVERAGE_FOLDER)?;
    }
    fs::create_dir_all(COVERAGE_FOLDER)?;
    
    cmd!("find", ".", "-name", "*.gcda", "-exec", "rm", "{}", ";").run()?;

    println!("=== running coverage ===");

    cmd!("cargo", "test")
        .env("RUSTC_BOOTSTRAP", "1")
        .env("CARGO_INCREMENTAL", "0")
        .env("RUSTFLAGS", "-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort")
        .env("RUSTDOCFLAGS", "-Cpanic=abort")
        .run()?;

    println!("Ok");

    println!("=== generating report ===");
    let (fmt, file) = match format {
        CoverageFormat::Html => ("html", "coverage/html"),
        CoverageFormat::Lcov => ("lcov", "coverage/report.lcov"),
    };
    let result = cmd!(
        "grcov",
        ".",
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
        "-o",
        file,
    )
    .run();

    match result {
        Ok(_) => {
            println!("Ok");
            println!("report location: {file}");
        }
        Err(err) => println!("KO: {}", err),
    }

    //cmd!("find", ".", "-name", "*.profraw", "-exec", "rm", "{}", ";").run()?;

    Ok(())
}
