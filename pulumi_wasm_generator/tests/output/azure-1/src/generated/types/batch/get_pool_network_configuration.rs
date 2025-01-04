#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPoolNetworkConfiguration {
    #[builder(into)]
    #[serde(rename = "acceleratedNetworkingEnabled")]
    pub r#accelerated_networking_enabled: Box<bool>,
    /// The scope of dynamic vnet assignment.
    #[builder(into)]
    #[serde(rename = "dynamicVnetAssignmentScope")]
    pub r#dynamic_vnet_assignment_scope: Box<String>,
    /// The inbound NAT pools that are used to address specific ports on the individual compute node externally.
    #[builder(into)]
    #[serde(rename = "endpointConfigurations")]
    pub r#endpoint_configurations: Box<Vec<super::super::types::batch::GetPoolNetworkConfigurationEndpointConfiguration>>,
    /// Type of public IP address provisioning.
    #[builder(into)]
    #[serde(rename = "publicAddressProvisioningType")]
    pub r#public_address_provisioning_type: Box<String>,
    /// A list of public IP ids that will be allocated to nodes.
    #[builder(into)]
    #[serde(rename = "publicIps")]
    pub r#public_ips: Box<Vec<String>>,
    /// The ARM resource identifier of the virtual network subnet which the compute nodes of the pool are joined too.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
