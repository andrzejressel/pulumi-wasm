#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LifecyclePolicyPolicyDetailExclusionRulesAmisLastLaunched {
    /// Defines the unit of time that the lifecycle policy uses to calculate elapsed time since the last instance launched from the AMI. For example: days, weeks, months, or years. Valid values: `DAYS`, `WEEKS`, `MONTHS` or `YEARS`.
    #[builder(into)]
    #[serde(rename = "unit")]
    pub r#unit: Box<String>,
    /// The integer number of units for the time period. For example 6 (months).
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<i32>,
}
