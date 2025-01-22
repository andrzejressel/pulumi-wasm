use assert_cmd::prelude::*;
use serde_json::Value;
use std::process::Command;
use std::str;

pub struct Stack {
    value: Value,
}

impl Stack {
    pub fn get_string(&self, pointer: &str) -> Result<&str, anyhow::Error> {
        self.value
            .pointer(pointer)
            .ok_or(anyhow!("Cannot find [{}] in stack export", pointer))?
            .as_str()
            .ok_or(anyhow!("[{}] is not a string", pointer))
    }
}

pub fn get_string(pointer: &str) -> &str {
    pointer
}

pub fn init_stack(stack_name: &str, github_token_env_vars: Vec<(String, String)>) -> Result<(), anyhow::Error> {
    Command::new("pulumi")
        .args(["stack", "init", stack_name])
        .env("PULUMI_CONFIG_PASSPHRASE", " ")
        .envs(github_token_env_vars.clone())
        .current_dir(".")
        .output()?;
    Ok(())
}

pub fn select_stack(stack_name: &str) -> Result<(), anyhow::Error> {
    Command::new("pulumi")
        .args(["stack", "select", stack_name])
        .current_dir(".")
        .assert()
        .success();
    Ok(())
}

pub fn up_stack(github_token_env_vars: Vec<(String, String)>) -> Result<(), anyhow::Error> {
    Command::new("pulumi")
        .args(["up", "-y"])
        .current_dir(".")
        .env("PULUMI_CONFIG_PASSPHRASE", " ")
        .envs(github_token_env_vars)
        .assert()
        .success();
    Ok(())
}

pub fn export_stack() -> Result<Stack, anyhow::Error> {
    let binding = Command::new("pulumi")
        .args(["stack", "export"])
        .current_dir(".")
        .env("PULUMI_CONFIG_PASSPHRASE", " ")
        .assert()
        .success();
    let stack = &binding.get_output().stdout;
    let stack: Value = serde_json::from_str(str::from_utf8(stack)?)?;
    Ok(Stack { value: stack })
}
