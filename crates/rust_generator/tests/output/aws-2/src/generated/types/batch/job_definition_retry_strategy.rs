#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobDefinitionRetryStrategy {
    /// Number of times to move a job to the `RUNNABLE` status. You may specify between `1` and `10` attempts.
    #[builder(into, default)]
    #[serde(rename = "attempts")]
    pub r#attempts: Box<Option<i32>>,
    /// Evaluate on exit conditions under which the job should be retried or failed. If this parameter is specified, then the `attempts` parameter must also be specified. You may specify up to 5 configuration blocks.
    #[builder(into, default)]
    #[serde(rename = "evaluateOnExits")]
    pub r#evaluate_on_exits: Box<Option<Vec<super::super::types::batch::JobDefinitionRetryStrategyEvaluateOnExit>>>,
}
