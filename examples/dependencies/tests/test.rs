use pulumi_wasm_examples_common::{export_stack, init_stack, select_stack, up_stack};

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

    let _ = export_stack()?;

    Ok(())
}
