#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FunctionDeadLetterConfig {
    /// ARN of an SNS topic or SQS queue to notify when an invocation fails. If this option is used, the function's IAM role must be granted suitable access to write to the target object, which means allowing either the `sns:Publish` or `sqs:SendMessage` action on this ARN, depending on which service is targeted.
    #[builder(into)]
    #[serde(rename = "targetArn")]
    pub r#target_arn: Box<String>,
}
