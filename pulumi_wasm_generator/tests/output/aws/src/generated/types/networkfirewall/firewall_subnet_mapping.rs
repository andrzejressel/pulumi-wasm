#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirewallSubnetMapping {
    /// The subnet's IP address type. Valida values: `"DUALSTACK"`, `"IPV4"`.
    #[builder(into, default)]
    #[serde(rename = "ipAddressType")]
    pub r#ip_address_type: Box<Option<String>>,
    /// The unique identifier for the subnet.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}