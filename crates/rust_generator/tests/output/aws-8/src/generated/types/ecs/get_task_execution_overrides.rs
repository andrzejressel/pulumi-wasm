#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTaskExecutionOverrides {
    /// One or more container overrides that are sent to a task. See below.
    #[builder(into, default)]
    #[serde(rename = "containerOverrides")]
    pub r#container_overrides: Box<Option<Vec<super::super::types::ecs::GetTaskExecutionOverridesContainerOverride>>>,
    /// The CPU override for the task.
    #[builder(into, default)]
    #[serde(rename = "cpu")]
    pub r#cpu: Box<Option<String>>,
    /// Amazon Resource Name (ARN) of the task execution role override for the task.
    #[builder(into, default)]
    #[serde(rename = "executionRoleArn")]
    pub r#execution_role_arn: Box<Option<String>>,
    /// Elastic Inference accelerator override for the task. See below.
    #[builder(into, default)]
    #[serde(rename = "inferenceAcceleratorOverrides")]
    pub r#inference_accelerator_overrides: Box<Option<Vec<super::super::types::ecs::GetTaskExecutionOverridesInferenceAcceleratorOverride>>>,
    /// The memory override for the task.
    #[builder(into, default)]
    #[serde(rename = "memory")]
    pub r#memory: Box<Option<String>>,
    /// Amazon Resource Name (ARN) of the role that containers in this task can assume.
    #[builder(into, default)]
    #[serde(rename = "taskRoleArn")]
    pub r#task_role_arn: Box<Option<String>>,
}
