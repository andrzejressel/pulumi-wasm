#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PolicyTargetTrackingConfigurationPredefinedMetricSpecification {
    /// Metric type.
    #[builder(into)]
    #[serde(rename = "predefinedMetricType")]
    pub r#predefined_metric_type: Box<String>,
    /// Identifies the resource associated with the metric type.
    #[builder(into, default)]
    #[serde(rename = "resourceLabel")]
    pub r#resource_label: Box<Option<String>>,
}
