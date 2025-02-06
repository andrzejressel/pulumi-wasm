#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobQueryDestinationEncryptionConfiguration {
    /// Describes the Cloud KMS encryption key that will be used to protect destination BigQuery table.
    /// The BigQuery Service Account associated with your project requires access to this encryption key.
    #[builder(into)]
    #[serde(rename = "kmsKeyName")]
    pub r#kms_key_name: Box<String>,
    /// (Output)
    /// Describes the Cloud KMS encryption key version used to protect destination BigQuery table.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyVersion")]
    pub r#kms_key_version: Box<Option<String>>,
}
