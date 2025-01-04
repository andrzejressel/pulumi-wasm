#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SelectionSelectionTag {
    /// The key in a key-value pair.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// An operation, such as `StringEquals`, that is applied to a key-value pair used to filter resources in a selection.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// The value in a key-value pair.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
