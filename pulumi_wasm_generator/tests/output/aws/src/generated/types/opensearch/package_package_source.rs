#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PackagePackageSource {
    /// The name of the Amazon S3 bucket containing the package.
    #[builder(into)]
    #[serde(rename = "s3BucketName")]
    pub r#s_3_bucket_name: Box<String>,
    /// Key (file name) of the package.
    #[builder(into)]
    #[serde(rename = "s3Key")]
    pub r#s_3_key: Box<String>,
}