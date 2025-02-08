#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipeTargetParametersEcsTaskParameters {
    /// List of capacity provider strategies to use for the task. If a capacityProviderStrategy is specified, the launchType parameter must be omitted. If no capacityProviderStrategy or launchType is specified, the defaultCapacityProviderStrategy for the cluster is used. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "capacityProviderStrategies")]
    pub r#capacity_provider_strategies: Box<Option<Vec<super::super::types::pipes::PipeTargetParametersEcsTaskParametersCapacityProviderStrategy>>>,
    /// Specifies whether to enable Amazon ECS managed tags for the task. Valid values: true, false.
    #[builder(into, default)]
    #[serde(rename = "enableEcsManagedTags")]
    pub r#enable_ecs_managed_tags: Box<Option<bool>>,
    /// Whether or not to enable the execute command functionality for the containers in this task. If true, this enables execute command functionality on all containers in the task. Valid values: true, false.
    #[builder(into, default)]
    #[serde(rename = "enableExecuteCommand")]
    pub r#enable_execute_command: Box<Option<bool>>,
    /// Specifies an Amazon ECS task group for the task. The maximum length is 255 characters.
    #[builder(into, default)]
    #[serde(rename = "group")]
    pub r#group: Box<Option<String>>,
    /// Specifies the launch type on which your task is running. The launch type that you specify here must match one of the launch type (compatibilities) of the target task. The FARGATE value is supported only in the Regions where AWS Fargate with Amazon ECS is supported. Valid Values: EC2, FARGATE, EXTERNAL
    #[builder(into, default)]
    #[serde(rename = "launchType")]
    pub r#launch_type: Box<Option<String>>,
    /// Use this structure if the Amazon ECS task uses the awsvpc network mode. This structure specifies the VPC subnets and security groups associated with the task, and whether a public IP address is to be used. This structure is required if LaunchType is FARGATE because the awsvpc mode is required for Fargate tasks. If you specify NetworkConfiguration when the target ECS task does not use the awsvpc network mode, the task fails. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "networkConfiguration")]
    pub r#network_configuration: Box<Option<super::super::types::pipes::PipeTargetParametersEcsTaskParametersNetworkConfiguration>>,
    /// The overrides that are associated with a task. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "overrides")]
    pub r#overrides: Box<Option<super::super::types::pipes::PipeTargetParametersEcsTaskParametersOverrides>>,
    /// An array of placement constraint objects to use for the task. You can specify up to 10 constraints per task (including constraints in the task definition and those specified at runtime). Detailed below.
    #[builder(into, default)]
    #[serde(rename = "placementConstraints")]
    pub r#placement_constraints: Box<Option<Vec<super::super::types::pipes::PipeTargetParametersEcsTaskParametersPlacementConstraint>>>,
    /// The placement strategy objects to use for the task. You can specify a maximum of five strategy rules per task. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "placementStrategies")]
    pub r#placement_strategies: Box<Option<Vec<super::super::types::pipes::PipeTargetParametersEcsTaskParametersPlacementStrategy>>>,
    /// Specifies the platform version for the task. Specify only the numeric portion of the platform version, such as 1.1.0. This structure is used only if LaunchType is FARGATE.
    #[builder(into, default)]
    #[serde(rename = "platformVersion")]
    pub r#platform_version: Box<Option<String>>,
    /// Specifies whether to propagate the tags from the task definition to the task. If no value is specified, the tags are not propagated. Tags can only be propagated to the task during task creation. To add tags to a task after task creation, use the TagResource API action. Valid Values: TASK_DEFINITION
    #[builder(into, default)]
    #[serde(rename = "propagateTags")]
    pub r#propagate_tags: Box<Option<String>>,
    /// The reference ID to use for the task. Maximum length of 1,024.
    #[builder(into, default)]
    #[serde(rename = "referenceId")]
    pub r#reference_id: Box<Option<String>>,
    /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<std::collections::HashMap<String, String>>>,
    /// The number of tasks to create based on TaskDefinition. The default is 1.
    #[builder(into, default)]
    #[serde(rename = "taskCount")]
    pub r#task_count: Box<Option<i32>>,
    /// The ARN of the task definition to use if the event target is an Amazon ECS task.
    #[builder(into)]
    #[serde(rename = "taskDefinitionArn")]
    pub r#task_definition_arn: Box<String>,
}
