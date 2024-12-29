#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum StorageType {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "gp2")]
    GP2,
    #[serde(rename = "gp3")]
    GP3,
    #[serde(rename = "io1")]
    IO1,
}
