#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DeliveryPipelineSerialPipelineStageStrategyCanaryCustomCanaryDeploymentPhaseConfig {
    /// Required. Percentage deployment for the phase.
    #[builder(into)]
    #[serde(rename = "percentage")]
    pub r#percentage: Box<i32>,
    /// Required. The ID to assign to the `Rollout` phase. This value must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: `^a-z?$`.
    #[builder(into)]
    #[serde(rename = "phaseId")]
    pub r#phase_id: Box<String>,
    /// Optional. Configuration for the postdeploy job of this phase. If this is not configured, postdeploy job will not be present for this phase.
    #[builder(into, default)]
    #[serde(rename = "postdeploy")]
    pub r#postdeploy: Box<Option<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyCanaryCustomCanaryDeploymentPhaseConfigPostdeploy>>,
    /// Optional. Configuration for the predeploy job of this phase. If this is not configured, predeploy job will not be present for this phase.
    #[builder(into, default)]
    #[serde(rename = "predeploy")]
    pub r#predeploy: Box<Option<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyCanaryCustomCanaryDeploymentPhaseConfigPredeploy>>,
    /// Skaffold profiles to use when rendering the manifest for this phase. These are in addition to the profiles list specified in the `DeliveryPipeline` stage.
    #[builder(into, default)]
    #[serde(rename = "profiles")]
    pub r#profiles: Box<Option<Vec<String>>>,
    /// Whether to run verify tests after the deployment.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "verify")]
    pub r#verify: Box<Option<bool>>,
}
