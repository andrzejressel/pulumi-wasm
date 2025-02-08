#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EventHubNamespaceNetworkRulesetsVirtualNetworkRule {
    /// Are missing virtual network service endpoints ignored?
    #[builder(into, default)]
    #[serde(rename = "ignoreMissingVirtualNetworkServiceEndpoint")]
    pub r#ignore_missing_virtual_network_service_endpoint: Box<Option<bool>>,
    /// The id of the subnet to match on.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
