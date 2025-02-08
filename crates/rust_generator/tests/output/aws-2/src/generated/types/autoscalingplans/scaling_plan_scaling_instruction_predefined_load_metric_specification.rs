#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ScalingPlanScalingInstructionPredefinedLoadMetricSpecification {
    /// Metric type. Valid values: `ALBTargetGroupRequestCount`, `ASGTotalCPUUtilization`, `ASGTotalNetworkIn`, `ASGTotalNetworkOut`.
    #[builder(into)]
    #[serde(rename = "predefinedLoadMetricType")]
    pub r#predefined_load_metric_type: Box<String>,
    /// Identifies the resource associated with the metric type.
    #[builder(into, default)]
    #[serde(rename = "resourceLabel")]
    pub r#resource_label: Box<Option<String>>,
}
