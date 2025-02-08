#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterNodePoolNetworkConfigAdditionalPodNetworkConfig {
    /// The maximum number of pods per node which use this pod network.
    #[builder(into, default)]
    #[serde(rename = "maxPodsPerNode")]
    pub r#max_pods_per_node: Box<Option<i32>>,
    /// The name of the secondary range on the subnet which provides IP address for this pod range.
    #[builder(into, default)]
    #[serde(rename = "secondaryPodRange")]
    pub r#secondary_pod_range: Box<Option<String>>,
    /// The name or self_link of the Google Compute Engine
    /// subnetwork in which the cluster's instances are launched.
    #[builder(into, default)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Box<Option<String>>,
}
