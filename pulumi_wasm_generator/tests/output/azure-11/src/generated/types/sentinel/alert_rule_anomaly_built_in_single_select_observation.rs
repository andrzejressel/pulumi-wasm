#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AlertRuleAnomalyBuiltInSingleSelectObservation {
    /// The description of the threshold observation.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The Name of the built-in Anomaly Alert Rule.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// A list of supported values of the single select observation.
    #[builder(into, default)]
    #[serde(rename = "supportedValues")]
    pub r#supported_values: Box<Option<Vec<String>>>,
    /// The value of the threshold observation.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
