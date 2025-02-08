#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetSecretReplicationUserManagedReplica {
    /// Customer Managed Encryption for the secret.
    #[builder(into)]
    #[serde(rename = "customerManagedEncryptions")]
    pub r#customer_managed_encryptions: Box<Vec<super::super::types::secretmanager::GetSecretReplicationUserManagedReplicaCustomerManagedEncryption>>,
    /// The canonical IDs of the location to replicate data. For example: "us-east1".
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
}
