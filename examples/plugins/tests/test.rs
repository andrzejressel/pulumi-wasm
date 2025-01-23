use assert_cmd::prelude::*;
use pulumi_wasm_examples_common::{init_stack, select_stack};
use std::process::Command;
use std::str;

#[test]
#[cfg_attr(not(feature = "example_test"), ignore)]
fn test_integration() -> Result<(), anyhow::Error> {
    let github_token_env_vars = if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        vec![("GITHUB_TOKEN".to_string(), token)]
    } else {
        vec![]
    };

    init_stack("test", &github_token_env_vars)?;
    select_stack("test")?;

    let command_up = Command::new("pulumi")
        .args(["up", "-y", "-v=5", "--logtostderr", "--logflow"])
        .current_dir(".")
        .env("PULUMI_CONFIG_PASSPHRASE", " ")
        .envs(github_token_env_vars)
        .assert()
        .success();

    let stack = &command_up.get_output().stderr;
    let str = str::from_utf8(stack)?;

    assert!(
        str.contains(
            "GetRequiredPlugins: plugins=[name:\"random\"  kind:\"resource\"  version:\"4.15.0\"]"
        ) || str.contains(
            "GetRequiredPlugins: plugins=[name:\"random\" kind:\"resource\" version:\"4.15.0\"]"
        ),
        "random plugin not found in stack output [{}]",
        str
    );

    Ok(())
}
