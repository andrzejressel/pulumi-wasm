#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AutomationRuleCriteriaUpdatedAtDateRange {
    /// A date range unit for the date filter. Valid values: `DAYS`.
    #[builder(into)]
    #[serde(rename = "unit")]
    pub r#unit: Box<String>,
    /// A date range value for the date filter, provided as an Integer.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<i32>,
}
