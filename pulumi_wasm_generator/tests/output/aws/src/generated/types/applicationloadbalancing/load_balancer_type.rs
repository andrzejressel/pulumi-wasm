#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum LoadBalancerType {
    #[serde(rename = "application")]
    Application,
    #[serde(rename = "network")]
    Network,
}
