#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SelectionConditionStringEqual {
    /// The key in a key-value pair.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// The value in a key-value pair.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
