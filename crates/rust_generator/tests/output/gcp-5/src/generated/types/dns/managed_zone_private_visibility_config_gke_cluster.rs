#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ManagedZonePrivateVisibilityConfigGkeCluster {
    /// The resource name of the cluster to bind this ManagedZone to.
    /// This should be specified in the format like
    /// `projects/*/locations/*/clusters/*`
    #[builder(into)]
    #[serde(rename = "gkeClusterName")]
    pub r#gke_cluster_name: Box<String>,
}
