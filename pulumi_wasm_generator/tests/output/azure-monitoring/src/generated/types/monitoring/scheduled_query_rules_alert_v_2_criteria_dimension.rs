#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ScheduledQueryRulesAlertV2CriteriaDimension {
    /// Name of the dimension.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Operator for dimension values. Possible values are `Exclude`,and `Include`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
    /// List of dimension values. Use a wildcard `*` to collect all.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}