use clap::Parser;
use error::RCatError;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

mod error;

const BUFFER_SIZE: usize = 512;

/// Concatenate FILE(s) to standard output.
///
/// rcat is a Rust implementation of cat(1)
#[derive(Parser)]
struct RCatArgs {
    /// File(s) to concatenante to standard output.
    file: Vec<std::path::PathBuf>,
}

trait SourceInput {
    fn display(&self) -> Result<(), RCatError>;
}

#[derive(Debug)]
struct FileInput {
    path: PathBuf,
}

impl FileInput {
    fn new(path: PathBuf) -> FileInput {
        FileInput { path }
    }

    fn get_file_path(&self) -> String {
        self.path.display().to_string()
    }

    fn open_file(&self) -> Result<File, RCatError> {
        Ok(File::open(&self.path)
            .map_err(|err| RCatError::new(self.get_file_path(), err.to_string()))?)
    }
}

impl SourceInput for FileInput {
    fn display(&self) -> Result<(), RCatError> {
        let mut file = self.open_file()?;

        let mut buffer = [0; BUFFER_SIZE];

        loop {
            let read = file
                .read(&mut buffer)
                .map_err(|err| RCatError::new(self.get_file_path(), err.to_string()))?;

            print!(
                "{}",
                String::from_utf8(buffer[..read].to_vec())
                    .map_err(|err| RCatError::new(self.get_file_path(), err.to_string()))?
            );

            if read < BUFFER_SIZE {
                break;
            }
        }

        Ok(())
    }
}
fn main() {
    let args = RCatArgs::parse();

    for path in args.file {
        let source = FileInput::new(path);
        if let Err(err) = source.display() {
            eprintln!("{}", err);
        }
    }
}
