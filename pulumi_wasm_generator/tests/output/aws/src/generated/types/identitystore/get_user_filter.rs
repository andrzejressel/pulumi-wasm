#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetUserFilter {
    /// Attribute path that is used to specify which attribute name to search. Currently, `UserName` is the only valid attribute path.
    #[builder(into)]
    #[serde(rename = "attributePath")]
    pub r#attribute_path: Box<String>,
    /// Value for an attribute.
    #[builder(into)]
    #[serde(rename = "attributeValue")]
    pub r#attribute_value: Box<String>,
}
