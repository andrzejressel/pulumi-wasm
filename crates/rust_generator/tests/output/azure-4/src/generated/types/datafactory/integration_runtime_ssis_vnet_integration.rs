#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IntegrationRuntimeSsisVnetIntegration {
    /// Static public IP addresses for the Azure-SSIS Integration Runtime. The size must be 2.
    #[builder(into, default)]
    #[serde(rename = "publicIps")]
    pub r#public_ips: Box<Option<Vec<String>>>,
    /// id of the subnet to which the nodes of the Azure-SSIS Integration Runtime will be added.
    /// 
    /// > **NOTE** Only one of `subnet_id` and `subnet_name` can be specified. If `subnet_name` is specified, `vnet_id` must be provided.
    #[builder(into, default)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<Option<String>>,
    /// Name of the subnet to which the nodes of the Azure-SSIS Integration Runtime will be added.
    #[builder(into, default)]
    #[serde(rename = "subnetName")]
    pub r#subnet_name: Box<Option<String>>,
    /// ID of the virtual network to which the nodes of the Azure-SSIS Integration Runtime will be added.
    #[builder(into, default)]
    #[serde(rename = "vnetId")]
    pub r#vnet_id: Box<Option<String>>,
}
