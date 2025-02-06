#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInstanceAutoscalingConfig {
    /// Asymmetric autoscaling options for specific replicas.
    #[builder(into)]
    #[serde(rename = "asymmetricAutoscalingOptions")]
    pub r#asymmetric_autoscaling_options: Box<Vec<super::super::types::spanner::GetInstanceAutoscalingConfigAsymmetricAutoscalingOption>>,
    /// Defines scale in controls to reduce the risk of response latency
    /// and outages due to abrupt scale-in events. Users can define the minimum and
    /// maximum compute capacity allocated to the instance, and the autoscaler will
    /// only scale within that range. Users can either use nodes or processing
    /// units to specify the limits, but should use the same unit to set both the
    /// min_limit and max_limit.
    #[builder(into)]
    #[serde(rename = "autoscalingLimits")]
    pub r#autoscaling_limits: Box<Vec<super::super::types::spanner::GetInstanceAutoscalingConfigAutoscalingLimit>>,
    /// Defines scale in controls to reduce the risk of response latency
    /// and outages due to abrupt scale-in events
    #[builder(into)]
    #[serde(rename = "autoscalingTargets")]
    pub r#autoscaling_targets: Box<Vec<super::super::types::spanner::GetInstanceAutoscalingConfigAutoscalingTarget>>,
}
