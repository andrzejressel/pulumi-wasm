#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AgentDataSourceServerSideEncryptionConfiguration {
    /// ARN of the AWS KMS key used to encrypt the resource.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: Box<Option<String>>,
}