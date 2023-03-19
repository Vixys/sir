use clap::Parser;
use std::fs::File;
use std::io::Read;

const BUFFER_SIZE: usize = 512;

/// Concatenate FILE(s) to standard output.
/// 
/// rcat is a Rust implementation of cat
#[derive(Parser)]
struct RcatArgs {
    /// File(s) to concatenante to standard output.
    file: Vec<std::path::PathBuf>
}

fn print_file(file: &mut File) {
    let mut buffer = [0; BUFFER_SIZE];

    loop {
        let read = match file.read(&mut buffer) {
            Ok(nb) => nb,
            Err(err) => panic!("Unknown error while reading file: {}", err),
        };
        

        match String::from_utf8(buffer[..read].to_vec()){
            Ok(content) => print!("{}", content),
            Err(err) => panic!("Unknown error converting to UTF8: {}", err),
        }

        if read < BUFFER_SIZE {
            break;
        }
    }
}

fn main() {
    let args = RcatArgs::parse();

    for path in args.file {
        match File::open(&path) {
            Ok(mut file) => print_file(&mut file),
            Err(_) => eprintln!("rcat: {}: No such file or directory", path.display()),
        }
    }
}
