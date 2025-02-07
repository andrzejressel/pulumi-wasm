#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ManagedZonePrivateVisibilityConfig {
    /// The list of Google Kubernetes Engine clusters that can see this zone.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "gkeClusters")]
    pub r#gke_clusters: Box<Option<Vec<super::super::types::dns::ManagedZonePrivateVisibilityConfigGkeCluster>>>,
    #[builder(into, default)]
    #[serde(rename = "networks")]
    pub r#networks: Box<Option<Vec<super::super::types::dns::ManagedZonePrivateVisibilityConfigNetwork>>>,
}
