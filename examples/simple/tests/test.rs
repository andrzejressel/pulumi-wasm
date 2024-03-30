use anyhow::anyhow;
use assert_cmd::prelude::*;
use serde_json::Value;
use std::process::Command;
use std::str;

#[test]
fn test_integration() -> Result<(), anyhow::Error> {
    let github_token_env_vars = if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        vec![("GITHUB_TOKEN".to_string(), token)]
    } else {
        vec![]
    };

    Command::new("pulumi")
        .args(["stack", "init", "test"])
        .env("PULUMI_CONFIG_PASSPHRASE", " ")
        .envs(github_token_env_vars.clone())
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
        .envs(github_token_env_vars)
        .assert()
        .success();

    let binding = Command::new("pulumi")
        .args(["stack", "output", "--json"])
        .current_dir(".")
        .env("PULUMI_CONFIG_PASSPHRASE", " ")
        .assert()
        .success();
    let stack = &binding.get_output().stdout;

    let stack: Value = serde_json::from_str(str::from_utf8(stack)?)?;

    let result = stack
        .pointer("/result")
        .ok_or(anyhow!("Cannot find [result] in stack export"))?
        .as_str()
        .ok_or(anyhow!("[result] is not a string"))?;

    assert_eq!(result.len(), 36);

    Ok(())
}
