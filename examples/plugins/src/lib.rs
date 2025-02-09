use anyhow::Error;
use pulumi_gestalt_providers_random::random_string;
use pulumi_gestalt_providers_random::random_string::RandomStringArgs;
use pulumi_gestalt_rust::{PulumiContext};

#[cfg(target_arch = "wasm32")]
pulumi_gestalt_rust::pulumi_main!();

fn pulumi_main(context: &PulumiContext) -> Result<(), Error> {
    let _ = random_string::create(
        context,
        "test",
        RandomStringArgs::builder().length(16).build_struct(),
    );
    Ok(())
}
