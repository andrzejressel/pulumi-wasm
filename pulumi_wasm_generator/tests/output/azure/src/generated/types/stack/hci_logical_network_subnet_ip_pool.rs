#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HciLogicalNetworkSubnetIpPool {
    /// The IPv4 address of the end of the IP address pool. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "end")]
    pub r#end: Box<String>,
    /// The IPv4 address of the start of the IP address pool. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "start")]
    pub r#start: Box<String>,
}