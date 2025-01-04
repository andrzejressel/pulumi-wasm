#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DeliveryPipelineSerialPipelineStageStrategyCanaryCanaryDeploymentPredeploy {
    /// Optional. A sequence of skaffold custom actions to invoke during execution of the predeploy job.
    #[builder(into, default)]
    #[serde(rename = "actions")]
    pub r#actions: Box<Option<Vec<String>>>,
}
