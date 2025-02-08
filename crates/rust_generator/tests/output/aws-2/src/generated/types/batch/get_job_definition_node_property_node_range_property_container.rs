#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetJobDefinitionNodePropertyNodeRangePropertyContainer {
    /// The command that's passed to the container.
    #[builder(into)]
    #[serde(rename = "commands")]
    pub r#commands: Box<Vec<String>>,
    /// The environment variables to pass to a container.
    #[builder(into)]
    #[serde(rename = "environments")]
    pub r#environments: Box<Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerEnvironment>>,
    /// The amount of ephemeral storage to allocate for the task. This parameter is used to expand the total amount of ephemeral storage available, beyond the default amount, for tasks hosted on AWS Fargate.
    #[builder(into)]
    #[serde(rename = "ephemeralStorages")]
    pub r#ephemeral_storages: Box<Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerEphemeralStorage>>,
    /// The Amazon Resource Name (ARN) of the execution role that AWS Batch can assume. For jobs that run on Fargate resources, you must provide an execution role.
    #[builder(into)]
    #[serde(rename = "executionRoleArn")]
    pub r#execution_role_arn: Box<String>,
    /// The platform configuration for jobs that are running on Fargate resources. Jobs that are running on EC2 resources must not specify this parameter.
    #[builder(into)]
    #[serde(rename = "fargatePlatformConfigurations")]
    pub r#fargate_platform_configurations: Box<Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerFargatePlatformConfiguration>>,
    /// The image used to start a container.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: Box<String>,
    /// The instance type to use for a multi-node parallel job.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Box<String>,
    /// The Amazon Resource Name (ARN) of the IAM role that the container can assume for AWS permissions.
    #[builder(into)]
    #[serde(rename = "jobRoleArn")]
    pub r#job_role_arn: Box<String>,
    /// Linux-specific modifications that are applied to the container.
    #[builder(into)]
    #[serde(rename = "linuxParameters")]
    pub r#linux_parameters: Box<Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerLinuxParameter>>,
    /// The log configuration specification for the container.
    #[builder(into)]
    #[serde(rename = "logConfigurations")]
    pub r#log_configurations: Box<Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerLogConfiguration>>,
    /// The mount points for data volumes in your container.
    #[builder(into)]
    #[serde(rename = "mountPoints")]
    pub r#mount_points: Box<Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerMountPoint>>,
    /// The network configuration for jobs that are running on Fargate resources.
    #[builder(into)]
    #[serde(rename = "networkConfigurations")]
    pub r#network_configurations: Box<Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerNetworkConfiguration>>,
    /// When this parameter is true, the container is given elevated permissions on the host container instance (similar to the root user).
    #[builder(into)]
    #[serde(rename = "privileged")]
    pub r#privileged: Box<bool>,
    /// When this parameter is true, the container is given read-only access to its root file system.
    #[builder(into)]
    #[serde(rename = "readonlyRootFilesystem")]
    pub r#readonly_root_filesystem: Box<bool>,
    /// The type and amount of resources to assign to a container.
    #[builder(into)]
    #[serde(rename = "resourceRequirements")]
    pub r#resource_requirements: Box<Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerResourceRequirement>>,
    /// An object that represents the compute environment architecture for AWS Batch jobs on Fargate.
    #[builder(into)]
    #[serde(rename = "runtimePlatforms")]
    pub r#runtime_platforms: Box<Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerRuntimePlatform>>,
    /// The secrets for the container.
    #[builder(into)]
    #[serde(rename = "secrets")]
    pub r#secrets: Box<Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerSecret>>,
    /// A list of ulimits to set in the container.
    #[builder(into)]
    #[serde(rename = "ulimits")]
    pub r#ulimits: Box<Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerUlimit>>,
    /// The user name to use inside the container.
    #[builder(into)]
    #[serde(rename = "user")]
    pub r#user: Box<String>,
    /// A list of data volumes used in a job.
    #[builder(into)]
    #[serde(rename = "volumes")]
    pub r#volumes: Box<Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerVolume>>,
}
