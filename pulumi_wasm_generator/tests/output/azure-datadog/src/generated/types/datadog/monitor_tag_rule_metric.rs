#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MonitorTagRuleMetric {
    /// A `filter` block as defined below.
    /// 
    /// > **NOTE:** List of filtering tags to be used for capturing metrics. If empty, all resources will be captured. If only Exclude action is specified, the rules will apply to the list of all available resources. If Include actions are specified, the rules will only include resources with the associated tags.
    #[builder(into, default)]
    #[serde(rename = "filters")]
    pub r#filters: Box<Option<Vec<super::super::types::datadog::MonitorTagRuleMetricFilter>>>,
}
