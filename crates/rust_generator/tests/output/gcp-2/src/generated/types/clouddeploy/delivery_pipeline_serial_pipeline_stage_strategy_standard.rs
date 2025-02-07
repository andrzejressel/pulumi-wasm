#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DeliveryPipelineSerialPipelineStageStrategyStandard {
    /// Optional. Configuration for the postdeploy job. If this is not configured, postdeploy job will not be present.
    #[builder(into, default)]
    #[serde(rename = "postdeploy")]
    pub r#postdeploy: Box<Option<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyStandardPostdeploy>>,
    /// Optional. Configuration for the predeploy job. If this is not configured, predeploy job will not be present.
    #[builder(into, default)]
    #[serde(rename = "predeploy")]
    pub r#predeploy: Box<Option<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyStandardPredeploy>>,
    /// Whether to verify a deployment.
    #[builder(into, default)]
    #[serde(rename = "verify")]
    pub r#verify: Box<Option<bool>>,
}
