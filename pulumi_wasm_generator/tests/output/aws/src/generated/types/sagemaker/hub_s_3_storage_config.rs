#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HubS3StorageConfig {
    /// The Amazon S3 bucket prefix for hosting hub content.interface.
    #[builder(into, default)]
    #[serde(rename = "s3OutputPath")]
    pub r#s_3_output_path: Box<Option<String>>,
}