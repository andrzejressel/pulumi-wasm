use std::process::Command;
use assert_cmd::prelude::*;
use std::str;
use anyhow::anyhow;
use serde_json::Value;

#[test]
fn test_integration() -> Result<(), anyhow::Error> {

    Command::new("pulumi")
        .args(["stack", "init", "test"])
        .env("PULUMI_CONFIG_PASSPHRASE", " ")
        .current_dir(".")
        .output()?;

    Command::new("pulumi")
        .args(["stack", "select", "test"])
        .current_dir(".")
        .assert()
        .success();

    Command::new("pulumi")
        .args(["up", "-y"])
        .current_dir(".")
        .env("PULUMI_CONFIG_PASSPHRASE", " ")
        .assert()
        .success();

    let binding = Command::new("pulumi")
        .args(["stack", "output", "--json"])
        .current_dir(".")
        .env("PULUMI_CONFIG_PASSPHRASE", " ")
        .assert()
        .success();
    let stack = &binding
        .get_output()
        .stdout;

    let stack: Value = serde_json::from_str(str::from_utf8(stack)?)?;

    let result = stack.pointer("/result").ok_or(anyhow!("Cannot find [result] in stack export"))?.as_str().ok_or(anyhow!("[result] is not a string"))?;

    assert_eq!(result.len(), 36);

    Ok(())
}
