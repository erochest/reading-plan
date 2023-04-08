use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn test_run() {
    Command::cargo_bin(env!["CARGO_PKG_NAME"])
        .unwrap()
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_run_one_day() {
    Command::cargo_bin(env!["CARGO_PKG_NAME"])
        .unwrap()
        .arg("--start-date=2020-01-01")
        .arg("--end-date=2020-01-01")
        .arg("--start-page=4")
        .arg("--end-page=10")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"date\": \"2020-01-01\"")
        .and(predicate::str::contains("\"start_page\": 4"))
        .and(predicate::str::contains("\"page_count\": 6"))
        .and(predicate::str::contains("\"raw_page_count\": 6.0")));
}