#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PoolNetworkConfigurationEndpointConfigurationNetworkSecurityGroupRule {
    /// The action that should be taken for a specified IP address, subnet range or tag. Acceptable values are `Allow` and `Deny`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "access")]
    pub r#access: Box<String>,
    /// The priority for this rule. The value must be at least `150`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
    /// The source address prefix or tag to match for the rule. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sourceAddressPrefix")]
    pub r#source_address_prefix: Box<String>,
    /// The source port ranges to match for the rule. Valid values are `*` (for all ports 0 - 65535) or arrays of ports or port ranges (i.e. `100-200`). The ports should in the range of 0 to 65535 and the port ranges or ports can't overlap. If any other values are provided the request fails with HTTP status code 400. Default value will be `*`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "sourcePortRanges")]
    pub r#source_port_ranges: Box<Option<Vec<String>>>,
}
