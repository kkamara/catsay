use std::process::Command; // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use assert_cmd::cargo;
use predicates::prelude::*;

#[test]
fn run_with_defaults() {
  Command::new(
    cargo::cargo_bin!("catsay")
  )
    .assert()
    .success()
    .stdout(
      predicate::str::contains("Meow!")
    );
}

#[test]
fn fail_on_non_existing_file()
  -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new(
      cargo::cargo_bin!("catsay")
    );
    cmd.args(&["-f", "no/such/file.txt"])
      .assert()
      .failure();
    Ok(())
  }