#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FleetScalingConfiguration {
    #[builder(into, default)]
    #[serde(rename = "desiredCapacity")]
    pub r#desired_capacity: Box<Option<i32>>,
    /// Maximum number of instances in the ﬂeet when auto-scaling.
    #[builder(into, default)]
    #[serde(rename = "maxCapacity")]
    pub r#max_capacity: Box<Option<i32>>,
    /// Scaling type for a compute fleet. Valid value: `TARGET_TRACKING_SCALING`.
    #[builder(into, default)]
    #[serde(rename = "scalingType")]
    pub r#scaling_type: Box<Option<String>>,
    /// Configuration block. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "targetTrackingScalingConfigs")]
    pub r#target_tracking_scaling_configs: Box<Option<Vec<super::super::types::codebuild::FleetScalingConfigurationTargetTrackingScalingConfig>>>,
}
