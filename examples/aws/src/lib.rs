use anyhow::Error;
use pulumi_wasm_providers_aws_mini::s3::bucket;
use pulumi_wasm_providers_random::random_string;
use pulumi_wasm_providers_random::random_string::RandomStringArgs;
use pulumi_wasm_rust::{add_export, pulumi_main};

#[pulumi_main]
fn test_main() -> Result<(), Error> {
    let random = random_string::create(
        "test",
        RandomStringArgs::builder().length(16).special(false).upper(false).build_struct(),
    );

    let my_bucket = bucket::create(
        "bucket",
        bucket::BucketArgs::builder().bucket(random.result).build_struct(),
    );

    add_export("bucket_name", &my_bucket.bucket);

    Ok(())
}
