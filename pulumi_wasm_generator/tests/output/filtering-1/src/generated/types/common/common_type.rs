#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CommonType {
    #[builder(into, default)]
    #[serde(rename = "commonProperty")]
    pub r#common_property: Box<Option<bool>>,
}
