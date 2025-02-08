#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AiServicesNetworkAclsVirtualNetworkRule {
    /// Whether to ignore a missing Virtual Network Service Endpoint or not. Default to `false`.
    #[builder(into, default)]
    #[serde(rename = "ignoreMissingVnetServiceEndpoint")]
    pub r#ignore_missing_vnet_service_endpoint: Box<Option<bool>>,
    /// The ID of the subnet which should be able to access this AI Services Account.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
