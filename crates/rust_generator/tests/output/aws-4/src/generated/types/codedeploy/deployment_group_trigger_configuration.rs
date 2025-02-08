#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DeploymentGroupTriggerConfiguration {
    /// The event type or types for which notifications are triggered. Some values that are supported: `DeploymentStart`, `DeploymentSuccess`, `DeploymentFailure`, `DeploymentStop`, `DeploymentRollback`, `InstanceStart`, `InstanceSuccess`, `InstanceFailure`.  See [the CodeDeploy documentation](http://docs.aws.amazon.com/codedeploy/latest/userguide/monitoring-sns-event-notifications-create-trigger.html) for all possible values.
    #[builder(into)]
    #[serde(rename = "triggerEvents")]
    pub r#trigger_events: Box<Vec<String>>,
    /// The name of the notification trigger.
    #[builder(into)]
    #[serde(rename = "triggerName")]
    pub r#trigger_name: Box<String>,
    /// The ARN of the SNS topic through which notifications are sent.
    #[builder(into)]
    #[serde(rename = "triggerTargetArn")]
    pub r#trigger_target_arn: Box<String>,
}
