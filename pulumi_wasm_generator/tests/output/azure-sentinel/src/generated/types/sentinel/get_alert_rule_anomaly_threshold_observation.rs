#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAlertRuleAnomalyThresholdObservation {
    /// The description of the threshold observation.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// The max value of the threshold observation.
    #[builder(into)]
    #[serde(rename = "max")]
    pub r#max: Box<String>,
    /// The min value of the threshold observation.
    #[builder(into)]
    #[serde(rename = "min")]
    pub r#min: Box<String>,
    /// The guid of this Sentinel Alert Rule Template. Either `display_name` or `name` have to be specified.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The value of the threshold observation.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
