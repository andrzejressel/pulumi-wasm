use anyhow::Error;
use pulumi_gestalt_providers_random::random_bytes;
use pulumi_gestalt_rust::{Context, add_export, pulumi_combine};
use pulumi_gestalt_rust::{GestaltContext, GestaltOutput};

#[cfg(target_arch = "wasm32")]
pulumi_gestalt_rust::pulumi_main!();
#[allow(dead_code)]
fn pulumi_main(context: &Context) -> Result<(), Error> {
    let custom_secret = context.new_secret(&10);
    let non_secret = context.new_output(&1);

    let secret = random_bytes::create(
        context,
        "secret",
        random_bytes::RandomBytesArgs::builder()
            .length(custom_secret.clone())
            .build_struct(),
    );

    let secret_mapped = secret.base64.map(|_| "mapped_secret".to_string());
    let combined_with_secret = pulumi_combine!(non_secret, secret_mapped.clone())
        .map(|(a, b)| format!("[{}, \"{}\"]", a, b));

    add_export("secret_output", &secret.base64);
    add_export("secret_output_mapped", &secret_mapped);
    add_export("combined_with_secret", &combined_with_secret);
    add_export("custom_secret", &custom_secret);
    Ok(())
}
