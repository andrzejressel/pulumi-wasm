#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetInstanceAutoscalingConfigAsymmetricAutoscalingOptionOverride {
    /// A nested object resource.
    #[builder(into)]
    #[serde(rename = "autoscalingLimits")]
    pub r#autoscaling_limits: Box<Vec<super::super::types::spanner::GetInstanceAutoscalingConfigAsymmetricAutoscalingOptionOverrideAutoscalingLimit>>,
}
