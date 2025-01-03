#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterMasterUserSecret {
    /// ARN for the KMS encryption key. When specifying `kms_key_id`, `storage_encrypted` needs to be set to true.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<Option<String>>,
    /// Amazon Resource Name (ARN) of the secret.
    #[builder(into, default)]
    #[serde(rename = "secretArn")]
    pub r#secret_arn: Box<Option<String>>,
    /// Status of the secret. Valid Values: `creating` | `active` | `rotating` | `impaired`.
    #[builder(into, default)]
    #[serde(rename = "secretStatus")]
    pub r#secret_status: Box<Option<String>>,
}
