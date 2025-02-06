#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterMigrationSource {
    /// The host and port of the on-premises instance in host:port format
    #[builder(into, default)]
    #[serde(rename = "hostPort")]
    pub r#host_port: Box<Option<String>>,
    /// Place holder for the external source identifier(e.g DMS job name) that created the cluster.
    #[builder(into, default)]
    #[serde(rename = "referenceId")]
    pub r#reference_id: Box<Option<String>>,
    /// Type of migration source.
    #[builder(into, default)]
    #[serde(rename = "sourceType")]
    pub r#source_type: Box<Option<String>>,
}
