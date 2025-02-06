#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterNodePoolNetworkConfigAdditionalNodeNetworkConfig {
    /// Name of the VPC where the additional interface belongs.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Box<String>,
    /// Name of the subnetwork where the additional interface belongs.
    #[builder(into)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Box<String>,
}
