use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*;
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rcat")?;

    cmd.arg("test/file/doesnt/exist");

    cmd.assert().failure().stderr(predicate::str::contains(
        "rcat: test/file/doesnt/exist: No such file or directory (os error 2)\n",
    ));

    Ok(())
}

#[test]
fn display_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.touch()?;
    file.write_str("file_content\n")?;
    let mut cmd = Command::cargo_bin("rcat")?;

    cmd.arg(file.as_os_str());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("file_content\n"));

    Ok(())
}

#[test]
fn display_content_in_filewith_show_ends() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.touch()?;
    file.write_str("file_content\n")?;
    let mut cmd = Command::cargo_bin("rcat")?;

    cmd.arg("-E");
    cmd.arg(file.as_os_str());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("file_content$\n"));

    Ok(())
}

#[test]
fn display_content_in_multiple_files() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.touch()?;
    file.write_str("file_content\n")?;
    let file2 = assert_fs::NamedTempFile::new("sample2.txt")?;
    file2.touch()?;
    file2.write_str("file2_content\n")?;
    let mut cmd = Command::cargo_bin("rcat")?;

    cmd.arg(file.as_os_str()).arg(file2.as_os_str());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("file_content\nfile2_content\n"));

    Ok(())
}

#[test]
fn display_content_in_multiple_files_with_one_missing() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.touch()?;
    file.write_str("file_content\n")?;
    let file2 = assert_fs::NamedTempFile::new("sample2.txt")?;
    file2.touch()?;
    file2.write_str("file2_content\n")?;
    let mut cmd = Command::cargo_bin("rcat")?;

    cmd.arg(file.as_os_str())
        .arg("__does_not_exit__.txt")
        .arg(file2.as_os_str());

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(
            "rcat: __does_not_exit__.txt: No such file or directory (os error 2)\n",
        ))
        .stdout(predicate::str::contains("file_content\nfile2_content\n"));

    Ok(())
}
