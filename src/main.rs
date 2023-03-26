use clap::Parser;
use std::fs::File;
use std::io::{Read};
use std::path::PathBuf;

const BUFFER_SIZE: usize = 512;

/// Concatenate FILE(s) to standard output.
/// 
/// rcat is a Rust implementation of cat(1)
#[derive(Parser)]
struct RCatArgs {
    /// File(s) to concatenante to standard output.
    file: Vec<std::path::PathBuf>
}

#[derive(Debug)]
struct RCatError(String);

impl std::fmt::Display for RCatError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(self.0.as_str())
    }
}

fn print_file(file: &mut File) -> Result<(), RCatError> {
    let mut buffer = [0; BUFFER_SIZE];

    loop {
        let read = file.read(&mut buffer).map_err(|err| RCatError(err.to_string()))?;

        print!("{}", String::from_utf8(buffer[..read].to_vec()).map_err(|err| RCatError(err.to_string()))?);

        if read < BUFFER_SIZE {
            break;
        }
    }
    Ok(())
}

fn open_file(path: &PathBuf) -> Result<File, RCatError> {
    Ok(File::open(&path).map_err(|err| RCatError(err.to_string()))?)
}


fn execute(path: &PathBuf) -> Result<(), RCatError> {
    print_file(&mut open_file(&path)?)?;
    Ok(())
}
fn main() {
    let args = RCatArgs::parse();

    for path in args.file {
        if let Err(err) = execute(&path) {
            eprintln!("rcat: {}: {}", path.display(), err);
        }
    }
}
