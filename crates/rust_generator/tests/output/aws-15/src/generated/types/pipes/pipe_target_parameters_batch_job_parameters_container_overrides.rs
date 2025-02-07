#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipeTargetParametersBatchJobParametersContainerOverrides {
    /// List of commands to send to the container that overrides the default command from the Docker image or the task definition. You must also specify a container name.
    #[builder(into, default)]
    #[serde(rename = "commands")]
    pub r#commands: Box<Option<Vec<String>>>,
    /// The environment variables to send to the container. You can add new environment variables, which are added to the container at launch, or you can override the existing environment variables from the Docker image or the task definition. You must also specify a container name. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "environments")]
    pub r#environments: Box<Option<Vec<super::super::types::pipes::PipeTargetParametersBatchJobParametersContainerOverridesEnvironment>>>,
    /// The instance type to use for a multi-node parallel job. This parameter isn't applicable to single-node container jobs or jobs that run on Fargate resources, and shouldn't be provided.
    #[builder(into, default)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Box<Option<String>>,
    /// The type and amount of a resource to assign to a container, instead of the default value from the task definition. The only supported resource is a GPU. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "resourceRequirements")]
    pub r#resource_requirements: Box<Option<Vec<super::super::types::pipes::PipeTargetParametersBatchJobParametersContainerOverridesResourceRequirement>>>,
}
