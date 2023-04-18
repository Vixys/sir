use log::debug;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::error::RCatError;

const BUFFER_SIZE: usize = 512;

pub trait SourceInput {
    fn display(&self, writer: impl std::io::Write) -> Result<(), RCatError>;
}

pub struct FileInput {
    path: PathBuf,
}

impl FileInput {
    pub fn new(path: PathBuf) -> FileInput {
        FileInput { path }
    }

    fn get_file_path(&self) -> String {
        self.path.display().to_string()
    }

    fn open_file(&self) -> Result<File, RCatError> {
        debug!("{}: opening file", self.get_file_path());
        Ok(File::open(&self.path)
            .map_err(|err| RCatError::new(self.get_file_path(), err.to_string()))?)
    }
}

impl SourceInput for FileInput {
    fn display(&self, mut writer: impl std::io::Write) -> Result<(), RCatError> {
        let mut file = self.open_file()?;

        let mut buffer = [0; BUFFER_SIZE];

        loop {
            debug!("{}: reading file.", self.get_file_path());

            let read = file
                .read(&mut buffer)
                .map_err(|err| RCatError::new(self.get_file_path(), err.to_string()))?;

            debug!(
                "{}: writing to output. ({} bytes read)",
                self.get_file_path(),
                read
            );
            write!(
                writer,
                "{}",
                String::from_utf8(buffer[..read].to_vec())
                    .map_err(|err| RCatError::new(self.get_file_path(), err.to_string()))?
            )
            .map_err(|err| RCatError::new(self.get_file_path(), err.to_string()))?;

            if read < BUFFER_SIZE {
                break;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use assert_fs::prelude::*;
    use test_log::test;

    use super::*;

    #[test]
    fn new_succeed() {
        let _ = FileInput::new(PathBuf::from("/tmp"));
    }

    #[test]
    fn get_file_path_succeed() {
        let file = FileInput::new(PathBuf::from("/tmp"));

        let result = file.get_file_path();

        assert_eq!(result, String::from("/tmp"));
    }

    #[test]
    fn open_file_file_exist_succeed() {
        let file = assert_fs::NamedTempFile::new("sample.txt").unwrap();
        file.touch().unwrap();
        let file_input = FileInput::new(file.to_path_buf());

        let result = file_input.open_file();

        assert!(result.is_ok(), "[{}]", result.err().unwrap());
    }

    #[test]
    fn open_file_file_does_not_exist_succeed() {
        let file_input = FileInput::new(PathBuf::from("does_not_exit.txt"));

        let result = file_input.open_file();

        assert!(result.is_err());
    }

    #[test]
    fn display_file_content_succeed() {
        let file = assert_fs::NamedTempFile::new("sample.txt").unwrap();
        file.touch().unwrap();
        file.write_str("A multiline\nfile content\nhappy face\n")
            .unwrap();
        let mut output = Vec::new();
        let file_input = FileInput::new(file.to_path_buf());

        let result = file_input.display(&mut output);

        assert!(result.is_ok(), "[{}]", result.err().unwrap());
        assert_eq!(output, b"A multiline\nfile content\nhappy face\n");
    }
}
