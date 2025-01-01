#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AwsClusterControlPlaneConfigEncryption {
    /// The ARN of the AWS KMS key used to encrypt cluster configuration.
    #[builder(into)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: Box<String>,
}
