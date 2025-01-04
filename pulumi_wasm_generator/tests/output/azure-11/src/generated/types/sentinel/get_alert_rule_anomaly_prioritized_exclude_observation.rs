#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAlertRuleAnomalyPrioritizedExcludeObservation {
    /// The description of the threshold observation.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// The excluded value per `description`.
    #[builder(into)]
    #[serde(rename = "exclude")]
    pub r#exclude: Box<String>,
    /// The guid of this Sentinel Alert Rule Template. Either `display_name` or `name` have to be specified.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The prioritized value per `description`.
    #[builder(into)]
    #[serde(rename = "prioritize")]
    pub r#prioritize: Box<String>,
}
