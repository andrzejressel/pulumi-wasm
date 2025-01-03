#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetProductFilter {
    /// Product attribute name that you want to filter on.
    #[builder(into)]
    #[serde(rename = "field")]
    pub r#field: Box<String>,
    /// Product attribute value that you want to filter on.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
