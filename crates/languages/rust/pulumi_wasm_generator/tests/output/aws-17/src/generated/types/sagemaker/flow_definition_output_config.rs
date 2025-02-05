#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlowDefinitionOutputConfig {
    /// The Amazon Key Management Service (KMS) key ARN for server-side encryption.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<Option<String>>,
    /// The Amazon S3 path where the object containing human output will be made available.
    #[builder(into)]
    #[serde(rename = "s3OutputPath")]
    pub r#s_3_output_path: Box<String>,
}
