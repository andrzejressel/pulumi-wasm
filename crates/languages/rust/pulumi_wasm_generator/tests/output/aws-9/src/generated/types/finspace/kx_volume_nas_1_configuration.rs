#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KxVolumeNas1Configuration {
    /// The size of the network attached storage.
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Box<i32>,
    /// The type of the network attached storage.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
