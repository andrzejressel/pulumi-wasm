#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetSecretReplicationUserManaged {
    /// The list of Replicas for this Secret. Cannot be empty.
    #[builder(into)]
    #[serde(rename = "replicas")]
    pub r#replicas: Box<Vec<super::super::types::secretmanager::GetSecretReplicationUserManagedReplica>>,
}
