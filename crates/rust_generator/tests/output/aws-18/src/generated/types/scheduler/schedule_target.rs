#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ScheduleTarget {
    /// ARN of the target of this schedule, such as a SQS queue or ECS cluster. For universal targets, this is a [Service ARN specific to the target service](https://docs.aws.amazon.com/scheduler/latest/UserGuide/managing-targets-universal.html#supported-universal-targets).
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// Information about an Amazon SQS queue that EventBridge Scheduler uses as a dead-letter queue for your schedule. If specified, EventBridge Scheduler delivers failed events that could not be successfully delivered to a target to the queue. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "deadLetterConfig")]
    pub r#dead_letter_config: Box<Option<super::super::types::scheduler::ScheduleTargetDeadLetterConfig>>,
    /// Templated target type for the Amazon ECS [`RunTask`](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_RunTask.html) API operation. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "ecsParameters")]
    pub r#ecs_parameters: Box<Option<super::super::types::scheduler::ScheduleTargetEcsParameters>>,
    /// Templated target type for the EventBridge [`PutEvents`](https://docs.aws.amazon.com/eventbridge/latest/APIReference/API_PutEvents.html) API operation. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "eventbridgeParameters")]
    pub r#eventbridge_parameters: Box<Option<super::super::types::scheduler::ScheduleTargetEventbridgeParameters>>,
    /// Text, or well-formed JSON, passed to the target. Read more in [Universal target](https://docs.aws.amazon.com/scheduler/latest/UserGuide/managing-targets-universal.html).
    #[builder(into, default)]
    #[serde(rename = "input")]
    pub r#input: Box<Option<String>>,
    /// Templated target type for the Amazon Kinesis [`PutRecord`](https://docs.aws.amazon.com/kinesis/latest/APIReference/API_PutRecord.html) API operation. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "kinesisParameters")]
    pub r#kinesis_parameters: Box<Option<super::super::types::scheduler::ScheduleTargetKinesisParameters>>,
    /// Information about the retry policy settings. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "retryPolicy")]
    pub r#retry_policy: Box<Option<super::super::types::scheduler::ScheduleTargetRetryPolicy>>,
    /// ARN of the IAM role that EventBridge Scheduler will use for this target when the schedule is invoked. Read more in [Set up the execution role](https://docs.aws.amazon.com/scheduler/latest/UserGuide/setting-up.html#setting-up-execution-role).
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// Templated target type for the Amazon SageMaker [`StartPipelineExecution`](https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_StartPipelineExecution.html) API operation. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "sagemakerPipelineParameters")]
    pub r#sagemaker_pipeline_parameters: Box<Option<super::super::types::scheduler::ScheduleTargetSagemakerPipelineParameters>>,
    /// The templated target type for the Amazon SQS [`SendMessage`](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/APIReference/API_SendMessage.html) API operation. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "sqsParameters")]
    pub r#sqs_parameters: Box<Option<super::super::types::scheduler::ScheduleTargetSqsParameters>>,
}
