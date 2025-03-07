use pulumi_gestalt_examples_common::{export_stack, init_stack, select_stack, up_stack};

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

    init_stack("test", &github_token_env_vars)?;
    select_stack("test")?;
    up_stack(&github_token_env_vars)?;

    let stack = export_stack()?;

    let logs = stack.get_string("/logs")?;
    let image_id = stack.get_string("/image_id")?;
    let labels = stack.get_string("/labels")?;
    let repo_digest = stack.get_string("/repo_digest")?;

    assert!(logs.contains("Hello World!"));
    assert!(!image_id.is_empty());
    assert!(labels.contains("value_1"));
    assert!(repo_digest.starts_with("public.ecr.aws/ubuntu/ubuntu@sha256:"));

    Ok(())
}
