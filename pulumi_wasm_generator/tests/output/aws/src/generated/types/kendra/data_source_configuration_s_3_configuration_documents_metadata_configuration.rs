#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSourceConfigurationS3ConfigurationDocumentsMetadataConfiguration {
    /// A prefix used to filter metadata configuration files in the AWS S3 bucket. The S3 bucket might contain multiple metadata files. Use `s3_prefix` to include only the desired metadata files.
    #[builder(into, default)]
    #[serde(rename = "s3Prefix")]
    pub r#s_3_prefix: Box<Option<String>>,
}