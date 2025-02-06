#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterNodePoolNetworkConfigAdditionalNodeNetworkConfig {
    /// The name or self_link of the Google Compute Engine
    /// network to which the cluster is connected. For Shared VPC, set this to the self link of the
    /// shared network.
    #[builder(into, default)]
    #[serde(rename = "network")]
    pub r#network: Box<Option<String>>,
    /// The name or self_link of the Google Compute Engine
    /// subnetwork in which the cluster's instances are launched.
    #[builder(into, default)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Box<Option<String>>,
}
