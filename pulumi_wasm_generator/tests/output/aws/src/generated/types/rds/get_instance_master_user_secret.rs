#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInstanceMasterUserSecret {
    /// The Amazon Web Services KMS key identifier that is used to encrypt the secret.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<String>,
    /// The Amazon Resource Name (ARN) of the secret.
    #[builder(into)]
    #[serde(rename = "secretArn")]
    pub r#secret_arn: Box<String>,
    /// The status of the secret. Valid Values: `creating` | `active` | `rotating` | `impaired`.
    #[builder(into)]
    #[serde(rename = "secretStatus")]
    pub r#secret_status: Box<String>,
}