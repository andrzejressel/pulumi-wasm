#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSecretsSecretReplicationUserManagedReplica {
    /// Customer Managed Encryption for the secret.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "customerManagedEncryptions")]
    pub r#customer_managed_encryptions: Box<Vec<super::super::types::secretmanager::GetSecretsSecretReplicationUserManagedReplicaCustomerManagedEncryption>>,
    /// The canonical IDs of the location to replicate data.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
}
