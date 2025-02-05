use pulumi_gestalt_core_test_common::{
    export_stack, export_stack_secret, init_stack, select_stack, up_stack,
};

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
    let secret_stack = export_stack_secret()?;

    assert_eq!(stack.get_string("/secret_output")?, "[secret]");
    assert_eq!(secret_stack.get_string("/secret_output")?.len(), 16);

    assert_eq!(stack.get_string("/secret_output_mapped")?, "[secret]");
    assert_eq!(
        secret_stack.get_string("/secret_output_mapped")?,
        "mapped_secret"
    );

    assert_eq!(stack.get_string("/combined_with_secret")?, "[secret]");
    assert_eq!(
        secret_stack.get_string("/combined_with_secret")?,
        "[1, \"mapped_secret\"]"
    );

    assert_eq!(stack.get_string("/custom_secret")?, "[secret]");
    assert_eq!(secret_stack.get_i64("/custom_secret")?, 10);

    Ok(())
}
