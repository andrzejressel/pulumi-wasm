#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SecretReplicationUserManagedReplica {
    /// Customer Managed Encryption for the secret.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "customerManagedEncryption")]
    pub r#customer_managed_encryption: Box<Option<super::super::types::secretmanager::SecretReplicationUserManagedReplicaCustomerManagedEncryption>>,
    /// The canonical IDs of the location to replicate data. For example: "us-east1".
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
}
