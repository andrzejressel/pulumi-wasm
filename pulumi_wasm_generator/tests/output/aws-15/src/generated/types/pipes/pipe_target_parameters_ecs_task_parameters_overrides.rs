#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipeTargetParametersEcsTaskParametersOverrides {
    /// One or more container overrides that are sent to a task. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "containerOverrides")]
    pub r#container_overrides: Box<Option<Vec<super::super::types::pipes::PipeTargetParametersEcsTaskParametersOverridesContainerOverride>>>,
    /// The number of cpu units reserved for the container, instead of the default value from the task definition. You must also specify a container name.
    #[builder(into, default)]
    #[serde(rename = "cpu")]
    pub r#cpu: Box<Option<String>>,
    /// The ephemeral storage setting override for the task.  Detailed below.
    #[builder(into, default)]
    #[serde(rename = "ephemeralStorage")]
    pub r#ephemeral_storage: Box<Option<super::super::types::pipes::PipeTargetParametersEcsTaskParametersOverridesEphemeralStorage>>,
    /// The Amazon Resource Name (ARN) of the task execution IAM role override for the task.
    #[builder(into, default)]
    #[serde(rename = "executionRoleArn")]
    pub r#execution_role_arn: Box<Option<String>>,
    /// List of Elastic Inference accelerator overrides for the task. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "inferenceAcceleratorOverrides")]
    pub r#inference_accelerator_overrides: Box<Option<Vec<super::super::types::pipes::PipeTargetParametersEcsTaskParametersOverridesInferenceAcceleratorOverride>>>,
    /// The hard limit (in MiB) of memory to present to the container, instead of the default value from the task definition. If your container attempts to exceed the memory specified here, the container is killed. You must also specify a container name.
    #[builder(into, default)]
    #[serde(rename = "memory")]
    pub r#memory: Box<Option<String>>,
    /// The Amazon Resource Name (ARN) of the IAM role that containers in this task can assume. All containers in this task are granted the permissions that are specified in this role.
    #[builder(into, default)]
    #[serde(rename = "taskRoleArn")]
    pub r#task_role_arn: Box<Option<String>>,
}
