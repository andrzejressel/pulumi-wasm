#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FlowDefinitionHumanLoopConfig {
    /// The Amazon Resource Name (ARN) of the human task user interface.
    #[builder(into)]
    #[serde(rename = "humanTaskUiArn")]
    pub r#human_task_ui_arn: Box<String>,
    /// Defines the amount of money paid to an Amazon Mechanical Turk worker for each task performed. See Public Workforce Task Price details below.
    #[builder(into, default)]
    #[serde(rename = "publicWorkforceTaskPrice")]
    pub r#public_workforce_task_price: Box<Option<super::super::types::sagemaker::FlowDefinitionHumanLoopConfigPublicWorkforceTaskPrice>>,
    /// The length of time that a task remains available for review by human workers. Valid value range between `1` and `864000`.
    #[builder(into, default)]
    #[serde(rename = "taskAvailabilityLifetimeInSeconds")]
    pub r#task_availability_lifetime_in_seconds: Box<Option<i32>>,
    /// The number of distinct workers who will perform the same task on each object. Valid value range between `1` and `3`.
    #[builder(into)]
    #[serde(rename = "taskCount")]
    pub r#task_count: Box<i32>,
    /// A description for the human worker task.
    #[builder(into)]
    #[serde(rename = "taskDescription")]
    pub r#task_description: Box<String>,
    /// An array of keywords used to describe the task so that workers can discover the task.
    #[builder(into, default)]
    #[serde(rename = "taskKeywords")]
    pub r#task_keywords: Box<Option<Vec<String>>>,
    /// The amount of time that a worker has to complete a task. The default value is `3600` seconds.
    #[builder(into, default)]
    #[serde(rename = "taskTimeLimitInSeconds")]
    pub r#task_time_limit_in_seconds: Box<Option<i32>>,
    /// A title for the human worker task.
    #[builder(into)]
    #[serde(rename = "taskTitle")]
    pub r#task_title: Box<String>,
    /// The Amazon Resource Name (ARN) of the human task user interface. Amazon Resource Name (ARN) of a team of workers. For Public workforces see [AWS Docs](https://docs.aws.amazon.com/sagemaker/latest/dg/sms-workforce-management-public.html).
    #[builder(into)]
    #[serde(rename = "workteamArn")]
    pub r#workteam_arn: Box<String>,
}
