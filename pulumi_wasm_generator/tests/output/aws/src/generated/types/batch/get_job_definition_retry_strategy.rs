#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetJobDefinitionRetryStrategy {
    /// The number of times to move a job to the RUNNABLE status.
    #[builder(into)]
    #[serde(rename = "attempts")]
    pub r#attempts: Box<i32>,
    /// Array of up to 5 objects that specify the conditions where jobs are retried or failed.
    #[builder(into)]
    #[serde(rename = "evaluateOnExits")]
    pub r#evaluate_on_exits: Box<Vec<super::super::types::batch::GetJobDefinitionRetryStrategyEvaluateOnExit>>,
}