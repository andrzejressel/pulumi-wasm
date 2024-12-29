#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AssociationOutputLocation {
    /// The S3 bucket name.
    #[builder(into)]
    #[serde(rename = "s3BucketName")]
    pub r#s_3_bucket_name: Box<String>,
    /// The S3 bucket prefix. Results stored in the root if not configured.
    #[builder(into, default)]
    #[serde(rename = "s3KeyPrefix")]
    pub r#s_3_key_prefix: Box<Option<String>>,
    /// The S3 bucket region.
    /// 
    /// Targets specify what instance IDs or tags to apply the document to and has these keys:
    #[builder(into, default)]
    #[serde(rename = "s3Region")]
    pub r#s_3_region: Box<Option<String>>,
}
