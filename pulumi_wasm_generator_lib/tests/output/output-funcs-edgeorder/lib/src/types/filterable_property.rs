//! Different types of filters supported and its values.

#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct FilterableProperty {
    /// Values to be filtered.
    #[builder(into)]
    #[serde(rename = "supportedValues")]
    pub r#supported_values: Box<Vec<String>>,
    /// Type of product filter.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<pulumi_wasm_provider_common::OneOf2<String, crate::types::SupportedFilterTypes>>,
}
