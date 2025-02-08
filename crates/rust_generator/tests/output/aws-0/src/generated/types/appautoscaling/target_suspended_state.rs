#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TargetSuspendedState {
    /// Whether scale in by a target tracking scaling policy or a step scaling policy is suspended. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "dynamicScalingInSuspended")]
    pub r#dynamic_scaling_in_suspended: Box<Option<bool>>,
    /// Whether scale out by a target tracking scaling policy or a step scaling policy is suspended. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "dynamicScalingOutSuspended")]
    pub r#dynamic_scaling_out_suspended: Box<Option<bool>>,
    /// Whether scheduled scaling is suspended. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "scheduledScalingSuspended")]
    pub r#scheduled_scaling_suspended: Box<Option<bool>>,
}
