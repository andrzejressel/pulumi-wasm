#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ScalingPlanHostPool {
    /// The ID of the HostPool to assign the Scaling Plan to.
    #[builder(into)]
    #[serde(rename = "hostpoolId")]
    pub r#hostpool_id: Box<String>,
    /// Specifies if the scaling plan is enabled or disabled for the HostPool.
    #[builder(into)]
    #[serde(rename = "scalingPlanEnabled")]
    pub r#scaling_plan_enabled: Box<bool>,
}
