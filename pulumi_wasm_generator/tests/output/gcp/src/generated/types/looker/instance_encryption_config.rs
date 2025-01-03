#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceEncryptionConfig {
    /// Name of the customer managed encryption key (CMEK) in KMS.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyName")]
    pub r#kms_key_name: Box<Option<String>>,
    /// (Output)
    /// Full name and version of the CMEK key currently in use to encrypt Looker data.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyNameVersion")]
    pub r#kms_key_name_version: Box<Option<String>>,
    /// (Output)
    /// Status of the customer managed encryption key (CMEK) in KMS.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyState")]
    pub r#kms_key_state: Box<Option<String>>,
}
