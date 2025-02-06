#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuleRetentionPeriod {
    /// The unit of time in which the retention period is measured. Currently, only DAYS is supported.
    #[builder(into)]
    #[serde(rename = "retentionPeriodUnit")]
    pub r#retention_period_unit: Box<String>,
    /// The period value for which the retention rule is to retain resources. The period is measured using the unit specified for RetentionPeriodUnit.
    #[builder(into)]
    #[serde(rename = "retentionPeriodValue")]
    pub r#retention_period_value: Box<i32>,
}
