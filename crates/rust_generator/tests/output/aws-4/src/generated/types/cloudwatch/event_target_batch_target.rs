#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EventTargetBatchTarget {
    /// The size of the array, if this is an array batch job. Valid values are integers between 2 and 10,000.
    #[builder(into, default)]
    #[serde(rename = "arraySize")]
    pub r#array_size: Box<Option<i32>>,
    /// The number of times to attempt to retry, if the job fails. Valid values are 1 to 10.
    #[builder(into, default)]
    #[serde(rename = "jobAttempts")]
    pub r#job_attempts: Box<Option<i32>>,
    /// The ARN or name of the job definition to use if the event target is an AWS Batch job. This job definition must already exist.
    #[builder(into)]
    #[serde(rename = "jobDefinition")]
    pub r#job_definition: Box<String>,
    /// The name to use for this execution of the job, if the target is an AWS Batch job.
    #[builder(into)]
    #[serde(rename = "jobName")]
    pub r#job_name: Box<String>,
}
