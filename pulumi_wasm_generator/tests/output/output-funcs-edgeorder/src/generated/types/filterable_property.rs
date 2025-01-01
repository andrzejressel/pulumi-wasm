#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FilterableProperty {
    /// Values to be filtered.
    #[builder(into)]
    #[serde(rename = "supportedValues")]
    pub r#supported_values: Box<Vec<String>>,
    /// Type of product filter.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<pulumi_wasm_rust::OneOf2<String, super::types::SupportedFilterTypes>>,
}
