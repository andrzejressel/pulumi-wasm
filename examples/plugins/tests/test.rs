use assert_cmd::prelude::*;
use std::str;
use common::stack_utils::{init_stack, select_stack, up_stack, export_stack};

#[test]
#[cfg_attr(not(feature = "example_test"), ignore)]
fn test_integration() -> Result<(), anyhow::Error> {
    let github_token_env_vars = if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        vec![("GITHUB_TOKEN".to_string(), token)]
    } else {
        vec![]
    };

    init_stack("test", github_token_env_vars.clone())?;
    select_stack("test")?;
    up_stack(github_token_env_vars)?;

    let stack = export_stack()?;
    let str = stack.to_string();

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
