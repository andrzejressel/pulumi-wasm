use anyhow::anyhow;
use assert_cmd::prelude::*;
use common::stack_utils::{init_stack, select_stack, up_stack, export_stack};

#[test]
#[cfg_attr(not(feature = "example_test"), ignore)]
fn test_integration() -> Result<(), anyhow::Error> {
    if std::env::var("GITHUB_ACTIONS").is_ok() && !cfg!(target_os = "linux") {
        return Ok(());
    }

    let github_token_env_vars = if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        vec![("GITHUB_TOKEN".to_string(), token)]
    } else {
        vec![]
    };

    init_stack("test", github_token_env_vars.clone())?;
    select_stack("test")?;
    up_stack(github_token_env_vars)?;

    let stack = export_stack()?;

    let logs = stack.get_string("/logs")?;
    let image_id = stack.get_string("/image_id")?;
    let labels = stack.get_string("/labels")?;
    let remote_image_id = stack.get_string("/remote_image_id")?;

    assert!(logs.contains("Hello World!"));
    assert!(!image_id.is_empty());
    assert!(labels.contains("value_1"));
    assert!(remote_image_id.starts_with("public.ecr.aws/ubuntu/ubuntu@sha256:"));

    Ok(())
}
