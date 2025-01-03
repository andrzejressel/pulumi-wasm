#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSecretsSecretReplicationAuto {
    /// Customer Managed Encryption for the secret.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "customerManagedEncryptions")]
    pub r#customer_managed_encryptions: Box<Vec<super::super::types::secretmanager::GetSecretsSecretReplicationAutoCustomerManagedEncryption>>,
}
