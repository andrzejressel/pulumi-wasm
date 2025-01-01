#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct Batching {
    #[builder(into, default)]
    #[serde(rename = "enableBatching")]
    pub r#enable_batching: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "sendAfter")]
    pub r#send_after: Box<Option<String>>,
}
