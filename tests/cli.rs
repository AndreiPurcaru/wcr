use std::process::Command;
use assert_cmd::assert::OutputAssertExt;
use assert_cmd::prelude::CommandCargoExt;
use predicates::prelude::predicate;

#[test]
fn it_adds_two() {
    assert_eq!(2+2, 4);
}

#[test]
fn file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("wcr")?;

    cmd.arg("test/file/not/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Could not read file"));
    Ok(())
}