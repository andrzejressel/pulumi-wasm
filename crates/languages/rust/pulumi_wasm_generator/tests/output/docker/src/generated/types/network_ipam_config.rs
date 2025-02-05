#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkIpamConfig {
    /// Auxiliary IPv4 or IPv6 addresses used by Network driver
    #[builder(into, default)]
    #[serde(rename = "auxAddress")]
    pub r#aux_address: Box<Option<std::collections::HashMap<String, String>>>,
    /// The IP address of the gateway
    #[builder(into, default)]
    #[serde(rename = "gateway")]
    pub r#gateway: Box<Option<String>>,
    /// The ip range in CIDR form
    #[builder(into, default)]
    #[serde(rename = "ipRange")]
    pub r#ip_range: Box<Option<String>>,
    /// The subnet in CIDR form
    #[builder(into, default)]
    #[serde(rename = "subnet")]
    pub r#subnet: Box<Option<String>>,
}
