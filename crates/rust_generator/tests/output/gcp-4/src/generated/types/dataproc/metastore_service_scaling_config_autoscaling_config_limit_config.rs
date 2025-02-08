#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct MetastoreServiceScalingConfigAutoscalingConfigLimitConfig {
    /// The maximum scaling factor that the service will autoscale to. The default value is 6.0.
    #[builder(into, default)]
    #[serde(rename = "maxScalingFactor")]
    pub r#max_scaling_factor: Box<Option<f64>>,
    /// The minimum scaling factor that the service will autoscale to. The default value is 0.1.
    #[builder(into, default)]
    #[serde(rename = "minScalingFactor")]
    pub r#min_scaling_factor: Box<Option<f64>>,
}
