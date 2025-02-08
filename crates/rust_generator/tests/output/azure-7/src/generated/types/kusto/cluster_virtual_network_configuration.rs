#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterVirtualNetworkConfiguration {
    /// Data management's service public IP address resource id.
    #[builder(into)]
    #[serde(rename = "dataManagementPublicIpId")]
    pub r#data_management_public_ip_id: Box<String>,
    /// Engine service's public IP address resource id.
    #[builder(into)]
    #[serde(rename = "enginePublicIpId")]
    pub r#engine_public_ip_id: Box<String>,
    /// The subnet resource id.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
