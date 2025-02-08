#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DeliveryPipelineSerialPipelineStageStrategy {
    /// Canary deployment strategy provides progressive percentage based deployments to a Target.
    #[builder(into, default)]
    #[serde(rename = "canary")]
    pub r#canary: Box<Option<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyCanary>>,
    /// Standard deployment strategy executes a single deploy and allows verifying the deployment.
    #[builder(into, default)]
    #[serde(rename = "standard")]
    pub r#standard: Box<Option<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyStandard>>,
}
