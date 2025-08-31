use std::process::Command;
use assert_cmd::prelude::*;

#[test]
fn test_run_timer_for_one_second() {
    let mut cmd = Command::cargo_bin("rust_timer").unwrap();
    cmd.arg("-s")
        .arg("1")
        .assert()
        .success();
}
