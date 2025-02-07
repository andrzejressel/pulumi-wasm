use anyhow::Error;
use pulumi_gestalt_providers_random::random_bytes;
use pulumi_gestalt_rust::{add_export, pulumi_combine, pulumi_main, Output, PulumiContext};

pulumi_main!();

fn pulumi_main(context: &PulumiContext) -> Result<(), Error> {
    let custom_secret = Output::new_secret(context, &10);
    let non_secret = Output::new(context, &1);

    let secret = random_bytes::create(
        context,
        "secret",
        random_bytes::RandomBytesArgs::builder()
            .length(custom_secret)
            .build_struct(),
    );

    let secret_mapped = secret.base64.map(|_| "mapped_secret".to_string());
    let combined_with_secret =
        pulumi_combine!(non_secret, secret_mapped).map(|(a, b)| format!("[{}, \"{}\"]", a, b));

    add_export("secret_output", &secret.base64);
    add_export("secret_output_mapped", &secret_mapped);
    add_export("combined_with_secret", &combined_with_secret);
    add_export("custom_secret", &custom_secret);
    Ok(())
}
