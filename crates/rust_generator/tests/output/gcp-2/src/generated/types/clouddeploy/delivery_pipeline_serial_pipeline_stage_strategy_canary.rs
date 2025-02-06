#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DeliveryPipelineSerialPipelineStageStrategyCanary {
    /// Configures the progressive based deployment for a Target.
    #[builder(into, default)]
    #[serde(rename = "canaryDeployment")]
    pub r#canary_deployment: Box<Option<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyCanaryCanaryDeployment>>,
    /// Configures the progressive based deployment for a Target, but allows customizing at the phase level where a phase represents each of the percentage deployments.
    #[builder(into, default)]
    #[serde(rename = "customCanaryDeployment")]
    pub r#custom_canary_deployment: Box<Option<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyCanaryCustomCanaryDeployment>>,
    /// Optional. Runtime specific configurations for the deployment strategy. The runtime configuration is used to determine how Cloud Deploy will split traffic to enable a progressive deployment.
    #[builder(into, default)]
    #[serde(rename = "runtimeConfig")]
    pub r#runtime_config: Box<Option<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyCanaryRuntimeConfig>>,
}
