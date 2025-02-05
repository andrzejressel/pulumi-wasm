use pulumi_gestalt_core_test_common::{export_stack, init_stack, select_stack, up_stack};

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

    let result = stack.get_string("/result")?;
    let transformed_result = stack.get_string("/transformed_result")?;
    let number = stack.get_i64("/number")?;
    let logs = stack.get_string("/logs")?;

    assert!(logs.contains("Hello World!"));
    assert_eq!(result.len(), 36);
    assert_eq!(transformed_result, format!("Result: {}", result));
    assert_eq!(number, 0);

    Ok(())
}
