#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlowDefinitionHumanLoopActivationConfig {
    /// defines under what conditions SageMaker creates a human loop. See Human Loop Activation Conditions Config details below.
    #[builder(into, default)]
    #[serde(rename = "humanLoopActivationConditionsConfig")]
    pub r#human_loop_activation_conditions_config: Box<Option<super::super::types::sagemaker::FlowDefinitionHumanLoopActivationConfigHumanLoopActivationConditionsConfig>>,
}