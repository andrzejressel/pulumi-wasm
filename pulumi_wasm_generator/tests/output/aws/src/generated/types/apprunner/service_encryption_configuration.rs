#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceEncryptionConfiguration {
    /// ARN of the KMS key used for encryption.
    #[builder(into)]
    #[serde(rename = "kmsKey")]
    pub r#kms_key: Box<String>,
}
