#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyConsumedMemoryThreshold {
    /// The utilization triggering the scale-in operation in percent.
    #[builder(into)]
    #[serde(rename = "scaleIn")]
    pub r#scale_in: Box<i32>,
    /// The utilization triggering the scale-out operation in percent.
    #[builder(into)]
    #[serde(rename = "scaleOut")]
    pub r#scale_out: Box<i32>,
}
