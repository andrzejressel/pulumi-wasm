use anyhow::Error;
use pulumi_wasm_providers_random::random_string;
use pulumi_wasm_providers_random::random_string::RandomStringArgs;
use pulumi_wasm_rust::{pulumi_main, PulumiContext};

pulumi_main!();

fn pulumi_main(context: &PulumiContext) -> Result<(), Error> {
    let _ = random_string::create(
        context,
        "test",
        RandomStringArgs::builder().length(16).build_struct(),
    );
    Ok(())
}
