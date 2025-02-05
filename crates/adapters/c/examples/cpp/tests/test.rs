use pulumi_gestalt_core_test_common::{export_stack, init_stack, select_stack, up_stack};

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
    up_stack(&github_token_env_vars)?;

    let stack = export_stack()?;

    let result = stack.get_string("/result")?;
    let double_length = stack.get_i64("/double_length")?;
    let static_string = stack.get_string("/static_string")?;

    assert_eq!(result.len(), 16);
    assert_eq!(double_length, 32);
    assert_eq!(static_string, "my_string");

    Ok(())
}
