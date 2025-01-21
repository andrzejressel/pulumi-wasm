use anyhow::Error;
use pulumi_wasm_providers_random::random_bytes;
use pulumi_wasm_providers_random::random_string;
use pulumi_wasm_providers_random::random_string::RandomStringArgs;
use pulumi_wasm_rust::Output;
use pulumi_wasm_rust::ToOutput;
use pulumi_wasm_rust::{add_export, pulumi_combine, pulumi_format, pulumi_main};

#[pulumi_main]
fn test_main() -> Result<(), Error> {
    let secret = random_bytes::create(
        "secret",
        random_bytes::RandomBytesArgs::builder()
            .length(10)
            .build_struct(),
    );

    let a = secret.hex.map(|hex| 1);

    add_export("secret", &secret.hex);
    add_export("A", &a);
    Ok(())
}
