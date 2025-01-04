#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointConfigurationProductionVariantCoreDumpConfig {
    /// The Amazon S3 bucket to send the core dump to.
    #[builder(into)]
    #[serde(rename = "destinationS3Uri")]
    pub r#destination_s_3_uri: Box<String>,
    /// The Amazon Web Services Key Management Service (Amazon Web Services KMS) key that SageMaker uses to encrypt the core dump data at rest using Amazon S3 server-side encryption.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<Option<String>>,
}
