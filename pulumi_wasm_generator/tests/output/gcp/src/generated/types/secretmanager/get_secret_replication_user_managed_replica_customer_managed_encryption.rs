#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSecretReplicationUserManagedReplicaCustomerManagedEncryption {
    /// Describes the Cloud KMS encryption key that will be used to protect destination secret.
    #[builder(into)]
    #[serde(rename = "kmsKeyName")]
    pub r#kms_key_name: Box<String>,
}
