#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PoolNetworkConfigurationEndpointConfiguration {
    /// The port number on the compute node. Acceptable values are between `1` and `65535` except for `29876`, `29877` as these are reserved. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "backendPort")]
    pub r#backend_port: Box<i32>,
    /// The range of external ports that will be used to provide inbound access to the backendPort on individual compute nodes in the format of `1000-1100`. Acceptable values range between `1` and `65534` except ports from `50000` to `55000` which are reserved by the Batch service. All ranges within a pool must be distinct and cannot overlap. Values must be a range of at least `100` nodes. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "frontendPortRange")]
    pub r#frontend_port_range: Box<String>,
    /// The name of the endpoint. The name must be unique within a Batch pool, can contain letters, numbers, underscores, periods, and hyphens. Names must start with a letter or number, must end with a letter, number, or underscore, and cannot exceed 77 characters. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A list of `network_security_group_rules` blocks as defined below that will be applied to the endpoint. The maximum number of rules that can be specified across all the endpoints on a Batch pool is `25`. If no network security group rules are specified, a default rule will be created to allow inbound access to the specified backendPort. Set as documented in the network_security_group_rules block below. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "networkSecurityGroupRules")]
    pub r#network_security_group_rules: Box<Option<Vec<super::super::types::batch::PoolNetworkConfigurationEndpointConfigurationNetworkSecurityGroupRule>>>,
    /// The protocol of the endpoint. Acceptable values are `TCP` and `UDP`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
}
