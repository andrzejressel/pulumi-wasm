#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipeTargetParametersBatchJobParameters {
    /// The array properties for the submitted job, such as the size of the array. The array size can be between 2 and 10,000. If you specify array properties for a job, it becomes an array job. This parameter is used only if the target is an AWS Batch job. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "arrayProperties")]
    pub r#array_properties: Box<Option<super::super::types::pipes::PipeTargetParametersBatchJobParametersArrayProperties>>,
    /// The overrides that are sent to a container. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "containerOverrides")]
    pub r#container_overrides: Box<Option<super::super::types::pipes::PipeTargetParametersBatchJobParametersContainerOverrides>>,
    /// A list of dependencies for the job. A job can depend upon a maximum of 20 jobs. You can specify a SEQUENTIAL type dependency without specifying a job ID for array jobs so that each child array job completes sequentially, starting at index 0. You can also specify an N_TO_N type dependency with a job ID for array jobs. In that case, each index child of this job must wait for the corresponding index child of each dependency to complete before it can begin. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "dependsOns")]
    pub r#depends_ons: Box<Option<Vec<super::super::types::pipes::PipeTargetParametersBatchJobParametersDependsOn>>>,
    /// The job definition used by this job. This value can be one of name, name:revision, or the Amazon Resource Name (ARN) for the job definition. If name is specified without a revision then the latest active revision is used.
    #[builder(into)]
    #[serde(rename = "jobDefinition")]
    pub r#job_definition: Box<String>,
    /// The name of the job. It can be up to 128 letters long.
    #[builder(into)]
    #[serde(rename = "jobName")]
    pub r#job_name: Box<String>,
    /// Additional parameters passed to the job that replace parameter substitution placeholders that are set in the job definition. Parameters are specified as a key and value pair mapping. Parameters included here override any corresponding parameter defaults from the job definition. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<std::collections::HashMap<String, String>>>,
    /// The retry strategy to use for failed jobs. When a retry strategy is specified here, it overrides the retry strategy defined in the job definition. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "retryStrategy")]
    pub r#retry_strategy: Box<Option<super::super::types::pipes::PipeTargetParametersBatchJobParametersRetryStrategy>>,
}
