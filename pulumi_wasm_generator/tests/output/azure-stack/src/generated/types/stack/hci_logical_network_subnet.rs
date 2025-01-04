#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HciLogicalNetworkSubnet {
    /// The address prefix in CIDR notation. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "addressPrefix")]
    pub r#address_prefix: Box<Option<String>>,
    /// The IP address allocation method for the subnet. Possible values are `Dynamic` and `Static`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "ipAllocationMethod")]
    pub r#ip_allocation_method: Box<String>,
    /// One or more `ip_pool` block as defined above. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** If `ip_pool` is not specified, it will be assigned by the server. If you experience a diff you may need to add this to `ignore_changes`.
    #[builder(into, default)]
    #[serde(rename = "ipPools")]
    pub r#ip_pools: Box<Option<Vec<super::super::types::stack::HciLogicalNetworkSubnetIpPool>>>,
    /// A `route` block as defined above. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "routes")]
    pub r#routes: Box<Option<Vec<super::super::types::stack::HciLogicalNetworkSubnetRoute>>>,
    /// The VLAN ID for the Logical Network. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "vlanId")]
    pub r#vlan_id: Box<Option<i32>>,
}
