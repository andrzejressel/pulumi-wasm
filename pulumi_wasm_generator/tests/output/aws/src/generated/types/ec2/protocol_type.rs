#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum ProtocolType {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "tcp")]
    TCP,
    #[serde(rename = "udp")]
    UDP,
    #[serde(rename = "icmp")]
    ICMP,
}
