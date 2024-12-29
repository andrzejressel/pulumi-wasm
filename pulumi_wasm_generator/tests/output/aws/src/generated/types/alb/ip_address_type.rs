#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum IpAddressType {
    #[serde(rename = "ipv4")]
    Ipv4,
    #[serde(rename = "dualstack")]
    Dualstack,
}
