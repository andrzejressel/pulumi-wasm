#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlexibleAppVersionAutomaticScalingCpuUtilization {
    /// Period of time over which CPU utilization is calculated.
    #[builder(into, default)]
    #[serde(rename = "aggregationWindowLength")]
    pub r#aggregation_window_length: Box<Option<String>>,
    /// Target CPU utilization ratio to maintain when scaling. Must be between 0 and 1.
    #[builder(into)]
    #[serde(rename = "targetUtilization")]
    pub r#target_utilization: Box<f64>,
}
