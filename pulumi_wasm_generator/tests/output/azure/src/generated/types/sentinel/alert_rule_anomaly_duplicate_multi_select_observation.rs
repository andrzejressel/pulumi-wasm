#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AlertRuleAnomalyDuplicateMultiSelectObservation {
    /// The description of the multi select observation.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The name of the multi select observation.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A list of supported values of the multi select observation.
    #[builder(into, default)]
    #[serde(rename = "supportedValues")]
    pub r#supported_values: Box<Option<Vec<String>>>,
    /// A list of values of the multi select observation.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}