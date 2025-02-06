#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LifecyclePolicyPolicyDetailsScheduleCreateRule {
    /// The schedule, as a Cron expression. The schedule interval must be between 1 hour and 1 year. Conflicts with `interval`, `interval_unit`, and `times`.
    #[builder(into, default)]
    #[serde(rename = "cronExpression")]
    pub r#cron_expression: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "interval")]
    pub r#interval: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "intervalUnit")]
    pub r#interval_unit: Box<Option<String>>,
    /// Specifies the destination for snapshots created by the policy. To create snapshots in the same Region as the source resource, specify `CLOUD`. To create snapshots on the same Outpost as the source resource, specify `OUTPOST_LOCAL`. If you omit this parameter, `CLOUD` is used by default. If the policy targets resources in an AWS Region, then you must create snapshots in the same Region as the source resource. If the policy targets resources on an Outpost, then you can create snapshots on the same Outpost as the source resource, or in the Region of that Outpost. Valid values are `CLOUD` and `OUTPOST_LOCAL`.
    #[builder(into, default)]
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    /// A list of times in 24 hour clock format that sets when the lifecycle policy should be evaluated. Max of 1. Conflicts with `cron_expression`. Must be set if `interval` is set.
    #[builder(into, default)]
    #[serde(rename = "times")]
    pub r#times: Box<Option<String>>,
}
