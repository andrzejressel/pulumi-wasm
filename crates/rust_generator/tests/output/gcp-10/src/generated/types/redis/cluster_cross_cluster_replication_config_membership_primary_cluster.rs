#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterCrossClusterReplicationConfigMembershipPrimaryCluster {
    /// The full resource path of the primary cluster in the format: projects/{project}/locations/{region}/clusters/{cluster-id}
    #[builder(into, default)]
    #[serde(rename = "cluster")]
    pub r#cluster: Box<Option<String>>,
    /// (Output)
    /// The unique id of the primary cluster.
    #[builder(into, default)]
    #[serde(rename = "uid")]
    pub r#uid: Box<Option<String>>,
}
