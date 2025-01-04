#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AutoscaleSettingProfileRule {
    /// A `metric_trigger` block as defined below.
    #[builder(into)]
    #[serde(rename = "metricTrigger")]
    pub r#metric_trigger: Box<super::super::types::monitoring::AutoscaleSettingProfileRuleMetricTrigger>,
    /// A `scale_action` block as defined below.
    #[builder(into)]
    #[serde(rename = "scaleAction")]
    pub r#scale_action: Box<super::super::types::monitoring::AutoscaleSettingProfileRuleScaleAction>,
}
