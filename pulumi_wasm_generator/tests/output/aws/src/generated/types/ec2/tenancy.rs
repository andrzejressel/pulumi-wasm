#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum Tenancy {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "dedicated")]
    Dedicated,
}
