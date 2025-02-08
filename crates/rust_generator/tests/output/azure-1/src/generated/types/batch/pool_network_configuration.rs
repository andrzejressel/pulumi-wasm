#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PoolNetworkConfiguration {
    /// Whether to enable accelerated networking. Possible values are `true` and `false`. Defaults to `false`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "acceleratedNetworkingEnabled")]
    pub r#accelerated_networking_enabled: Box<Option<bool>>,
    /// The scope of dynamic vnet assignment. Allowed values: `none`, `job`. Changing this forces a new resource to be created. Defaults to `none`.
    #[builder(into, default)]
    #[serde(rename = "dynamicVnetAssignmentScope")]
    pub r#dynamic_vnet_assignment_scope: Box<Option<String>>,
    /// A list of `endpoint_configuration` blocks that can be used to address specific ports on an individual compute node externally as defined below. Set as documented in the inbound_nat_pools block below. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "endpointConfigurations")]
    pub r#endpoint_configurations: Box<Option<Vec<super::super::types::batch::PoolNetworkConfigurationEndpointConfiguration>>>,
    /// Type of public IP address provisioning. Supported values are `BatchManaged`, `UserManaged` and `NoPublicIPAddresses`.
    #[builder(into, default)]
    #[serde(rename = "publicAddressProvisioningType")]
    pub r#public_address_provisioning_type: Box<Option<String>>,
    /// A list of public IP ids that will be allocated to nodes. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "publicIps")]
    pub r#public_ips: Box<Option<Vec<String>>>,
    /// The ARM resource identifier of the virtual network subnet which the compute nodes of the pool will join. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<Option<String>>,
}
