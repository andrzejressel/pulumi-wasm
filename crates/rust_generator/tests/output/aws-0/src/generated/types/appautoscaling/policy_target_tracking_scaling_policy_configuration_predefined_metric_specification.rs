#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicyTargetTrackingScalingPolicyConfigurationPredefinedMetricSpecification {
    /// Metric type.
    #[builder(into)]
    #[serde(rename = "predefinedMetricType")]
    pub r#predefined_metric_type: Box<String>,
    /// Reserved for future use if the `predefined_metric_type` is not `ALBRequestCountPerTarget`. If the `predefined_metric_type` is `ALBRequestCountPerTarget`, you must specify this argument. Documentation can be found at: [AWS Predefined Scaling Metric Specification](https://docs.aws.amazon.com/autoscaling/plans/APIReference/API_PredefinedScalingMetricSpecification.html). Must be less than or equal to 1023 characters in length.
    #[builder(into, default)]
    #[serde(rename = "resourceLabel")]
    pub r#resource_label: Box<Option<String>>,
}
