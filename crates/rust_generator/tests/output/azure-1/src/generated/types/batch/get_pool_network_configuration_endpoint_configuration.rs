#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPoolNetworkConfigurationEndpointConfiguration {
    /// The port number on the compute node.
    #[builder(into)]
    #[serde(rename = "backendPort")]
    pub r#backend_port: Box<i32>,
    /// The range of external ports that are used to provide inbound access to the backendPort on the individual compute nodes in the format of `1000-1100`.
    #[builder(into)]
    #[serde(rename = "frontendPortRange")]
    pub r#frontend_port_range: Box<String>,
    /// The name of the user account.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The list of network security group rules that are applied to the endpoint.
    #[builder(into)]
    #[serde(rename = "networkSecurityGroupRules")]
    pub r#network_security_group_rules: Box<Vec<super::super::types::batch::GetPoolNetworkConfigurationEndpointConfigurationNetworkSecurityGroupRule>>,
    /// The protocol of the endpoint.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
}
