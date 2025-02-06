#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GroupResourceQuery {
    /// The resource query as a JSON string.
    #[builder(into)]
    #[serde(rename = "query")]
    pub r#query: Box<String>,
    /// The type of the resource query. Defaults to `TAG_FILTERS_1_0`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
