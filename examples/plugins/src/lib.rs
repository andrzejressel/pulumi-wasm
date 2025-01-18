use anyhow::Error;
use pulumi_wasm_providers_random::random_string;
use pulumi_wasm_providers_random::random_string::RandomStringArgs;
use pulumi_wasm_rust::pulumi_main;

#[pulumi_main]
fn test_main() -> Result<(), Error> {
    let _ = random_string::create(
        "test",
        RandomStringArgs::builder().length(16).build_struct(),
    );
    Ok(())
}
