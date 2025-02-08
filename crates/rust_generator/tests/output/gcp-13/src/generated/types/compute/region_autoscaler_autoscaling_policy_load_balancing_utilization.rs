#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegionAutoscalerAutoscalingPolicyLoadBalancingUtilization {
    /// Fraction of backend capacity utilization (set in HTTP(s) load
    /// balancing configuration) that autoscaler should maintain. Must
    /// be a positive float value. If not defined, the default is 0.8.
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Box<f64>,
}
