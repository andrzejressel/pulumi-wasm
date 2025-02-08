#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetBackendAddressPoolBackendAddress {
    /// A list of `inbound_nat_rule_port_mapping` block as defined below.
    #[builder(into)]
    #[serde(rename = "inboundNatRulePortMappings")]
    pub r#inbound_nat_rule_port_mappings: Box<Vec<super::super::types::lb::GetBackendAddressPoolBackendAddressInboundNatRulePortMapping>>,
    /// The Static IP address for this Load Balancer within the Virtual Network.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<String>,
    /// Specifies the name of the Backend Address Pool.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The ID of the Virtual Network where the Backend Address of the Load Balancer exists.
    #[builder(into)]
    #[serde(rename = "virtualNetworkId")]
    pub r#virtual_network_id: Box<String>,
}
