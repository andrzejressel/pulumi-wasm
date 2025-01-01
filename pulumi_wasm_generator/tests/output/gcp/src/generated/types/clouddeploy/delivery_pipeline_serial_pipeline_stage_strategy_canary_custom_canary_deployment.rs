#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DeliveryPipelineSerialPipelineStageStrategyCanaryCustomCanaryDeployment {
    /// Required. Configuration for each phase in the canary deployment in the order executed.
    #[builder(into)]
    #[serde(rename = "phaseConfigs")]
    pub r#phase_configs: Box<Vec<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyCanaryCustomCanaryDeploymentPhaseConfig>>,
}
