#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TableEncryptionSpecification {
    /// The Amazon Resource Name (ARN) of the customer managed KMS key.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyIdentifier")]
    pub r#kms_key_identifier: Box<Option<String>>,
    /// The encryption option specified for the table. Valid values: `AWS_OWNED_KMS_KEY`, `CUSTOMER_MANAGED_KMS_KEY`. The default value is `AWS_OWNED_KMS_KEY`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
