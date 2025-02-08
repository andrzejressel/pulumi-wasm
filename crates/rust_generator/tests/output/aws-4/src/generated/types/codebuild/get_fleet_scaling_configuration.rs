#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetFleetScalingConfiguration {
    /// The desired number of instances in the ﬂeet when auto-scaling.
    #[builder(into)]
    #[serde(rename = "desiredCapacity")]
    pub r#desired_capacity: Box<i32>,
    /// The maximum number of instances in the ﬂeet when auto-scaling.
    #[builder(into)]
    #[serde(rename = "maxCapacity")]
    pub r#max_capacity: Box<i32>,
    /// The scaling type for a compute fleet.
    #[builder(into)]
    #[serde(rename = "scalingType")]
    pub r#scaling_type: Box<String>,
    /// Nested attribute containing information about thresholds when new instance is auto-scaled into the compute fleet.
    #[builder(into)]
    #[serde(rename = "targetTrackingScalingConfigs")]
    pub r#target_tracking_scaling_configs: Box<Vec<super::super::types::codebuild::GetFleetScalingConfigurationTargetTrackingScalingConfig>>,
}
