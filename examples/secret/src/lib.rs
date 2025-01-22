use anyhow::Error;
use pulumi_wasm_providers_random::random_bytes;
use pulumi_wasm_rust::{add_export, pulumi_main};

#[pulumi_main]
fn test_main() -> Result<(), Error> {
    let secret = random_bytes::create(
        "secret",
        random_bytes::RandomBytesArgs::builder()
            .length(10)
            .build_struct(),
    );

    let secret_mapped = secret.hex.map(|hex| 1);

    add_export("secret_output", &secret.hex);
    add_export("secret_output_mapped", &secret_mapped);
    Ok(())
}
