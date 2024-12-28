#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInfrastructureAccessTargetsTargetIpIpv4 {
    /// The IP address of the target.
    #[builder(into)]
    #[serde(rename = "ipAddr")]
    pub r#ip_addr: Box<String>,
    /// The private virtual network identifier for the target.
    #[builder(into)]
    #[serde(rename = "virtualNetworkId")]
    pub r#virtual_network_id: Box<String>,
}
