#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EmailRoutingCatchAllAction {
    /// Type of supported action. Available values: `drop`, `forward`, `worker`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    /// A list with items in the following form.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
