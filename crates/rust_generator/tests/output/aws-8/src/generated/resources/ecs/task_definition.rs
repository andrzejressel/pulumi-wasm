/// Manages a revision of an ECS task definition to be used in `aws.ecs.Service`.
///
/// ## Example Usage
///
/// ### Basic Example
///
/// ```yaml
/// resources:
///   service:
///     type: aws:ecs:TaskDefinition
///     properties:
///       family: service
///       containerDefinitions:
///         fn::toJSON:
///           - name: first
///             image: service-first
///             cpu: 10
///             memory: 512
///             essential: true
///             portMappings:
///               - containerPort: 80
///                 hostPort: 80
///           - name: second
///             image: service-second
///             cpu: 10
///             memory: 256
///             essential: true
///             portMappings:
///               - containerPort: 443
///                 hostPort: 443
///       volumes:
///         - name: service-storage
///           hostPath: /ecs/service-storage
///       placementConstraints:
///         - type: memberOf
///           expression: attribute:ecs.availability-zone in [us-west-2a, us-west-2b]
/// ```
///
/// ### With AppMesh Proxy
///
/// ```yaml
/// resources:
///   service:
///     type: aws:ecs:TaskDefinition
///     properties:
///       family: service
///       containerDefinitions:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: task-definitions/service.json
///           return: result
///       proxyConfiguration:
///         type: APPMESH
///         containerName: applicationContainerName
///         properties:
///           AppPorts: '8080'
///           EgressIgnoredIPs: 169.254.170.2,169.254.169.254
///           IgnoredUID: '1337'
///           ProxyEgressPort: 15001
///           ProxyIngressPort: 15000
/// ```
///
/// ### Example Using `docker_volume_configuration`
///
/// ```yaml
/// resources:
///   service:
///     type: aws:ecs:TaskDefinition
///     properties:
///       family: service
///       containerDefinitions:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: task-definitions/service.json
///           return: result
///       volumes:
///         - name: service-storage
///           dockerVolumeConfiguration:
///             scope: shared
///             autoprovision: true
///             driver: local
///             driverOpts:
///               type: nfs
///               device: ${fs.dnsName}:/
///               o: addr=${fs.dnsName},rsize=1048576,wsize=1048576,hard,timeo=600,retrans=2,noresvport
/// ```
///
/// ### Example Using `efs_volume_configuration`
///
/// ```yaml
/// resources:
///   service:
///     type: aws:ecs:TaskDefinition
///     properties:
///       family: service
///       containerDefinitions:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: task-definitions/service.json
///           return: result
///       volumes:
///         - name: service-storage
///           efsVolumeConfiguration:
///             fileSystemId: ${fs.id}
///             rootDirectory: /opt/data
///             transitEncryption: ENABLED
///             transitEncryptionPort: 2999
///             authorizationConfig:
///               accessPointId: ${test.id}
///               iam: ENABLED
/// ```
///
/// ### Example Using `fsx_windows_file_server_volume_configuration`
///
/// ```yaml
/// resources:
///   service:
///     type: aws:ecs:TaskDefinition
///     properties:
///       family: service
///       containerDefinitions:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: task-definitions/service.json
///           return: result
///       volumes:
///         - name: service-storage
///           fsxWindowsFileServerVolumeConfiguration:
///             fileSystemId: ${testAwsFsxWindowsFileSystem.id}
///             rootDirectory: \data
///             authorizationConfig:
///               credentialsParameter: ${test.arn}
///               domain: ${testAwsDirectoryServiceDirectory.name}
///   test:
///     type: aws:secretsmanager:SecretVersion
///     properties:
///       secretId: ${testAwsSecretsmanagerSecret.id}
///       secretString:
///         fn::toJSON:
///           username: admin
///           password: ${testAwsDirectoryServiceDirectory.password}
/// ```
///
/// ### Example Using `container_definitions` and `inference_accelerator`
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = task_definition::create(
///         "test",
///         TaskDefinitionArgs::builder()
///             .container_definitions(
///                 "[\n  {\n    \"cpu\": 10,\n    \"command\": [\"sleep\", \"10\"],\n    \"entryPoint\": [\"/\"],\n    \"environment\": [\n      {\"name\": \"VARNAME\", \"value\": \"VARVAL\"}\n    ],\n    \"essential\": true,\n    \"image\": \"jenkins\",\n    \"memory\": 128,\n    \"name\": \"jenkins\",\n    \"portMappings\": [\n      {\n        \"containerPort\": 80,\n        \"hostPort\": 8080\n      }\n    ],\n        \"resourceRequirements\":[\n            {\n                \"type\":\"InferenceAccelerator\",\n                \"value\":\"device_1\"\n            }\n        ]\n  }\n]\n",
///             )
///             .family("test")
///             .inference_accelerators(
///                 vec![
///                     TaskDefinitionInferenceAccelerator::builder().deviceName("device_1")
///                     .deviceType("eia1.medium").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Example Using `runtime_platform` and `fargate`
///
/// ```yaml
/// resources:
///   test:
///     type: aws:ecs:TaskDefinition
///     properties:
///       family: test
///       requiresCompatibilities:
///         - FARGATE
///       networkMode: awsvpc
///       cpu: 1024
///       memory: 2048
///       containerDefinitions: |
///         [
///           {
///             "name": "iis",
///             "image": "mcr.microsoft.com/windows/servercore/iis",
///             "cpu": 1024,
///             "memory": 2048,
///             "essential": true
///           }
///         ]
///       runtimePlatform:
///         operatingSystemFamily: WINDOWS_SERVER_2019_CORE
///         cpuArchitecture: X86_64
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ECS Task Definitions using their ARNs. For example:
///
/// ```sh
/// $ pulumi import aws:ecs/taskDefinition:TaskDefinition example arn:aws:ecs:us-east-1:012345678910:task-definition/mytaskfamily:123
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod task_definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TaskDefinitionArgs {
        /// A list of valid [container definitions](http://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_ContainerDefinition.html) provided as a single valid JSON document. Please note that you should only provide values that are part of the container definition document. For a detailed description of what parameters are available, see the [Task Definition Parameters](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_definition_parameters.html) section from the official [Developer Guide](https://docs.aws.amazon.com/AmazonECS/latest/developerguide).
        #[builder(into)]
        pub container_definitions: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Number of cpu units used by the task. If the `requires_compatibilities` is `FARGATE` this field is required.
        #[builder(into, default)]
        pub cpu: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The amount of ephemeral storage to allocate for the task. This parameter is used to expand the total amount of ephemeral storage available, beyond the default amount, for tasks hosted on AWS Fargate. See Ephemeral Storage.
        #[builder(into, default)]
        pub ephemeral_storage: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ecs::TaskDefinitionEphemeralStorage>,
        >,
        /// ARN of the task execution role that the Amazon ECS container agent and the Docker daemon can assume.
        #[builder(into, default)]
        pub execution_role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A unique name for your task definition.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub family: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block(s) with Inference Accelerators settings. Detailed below.
        #[builder(into, default)]
        pub inference_accelerators: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ecs::TaskDefinitionInferenceAccelerator>>,
        >,
        /// IPC resource namespace to be used for the containers in the task The valid values are `host`, `task`, and `none`.
        #[builder(into, default)]
        pub ipc_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amount (in MiB) of memory used by the task. If the `requires_compatibilities` is `FARGATE` this field is required.
        #[builder(into, default)]
        pub memory: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Docker networking mode to use for the containers in the task. Valid values are `none`, `bridge`, `awsvpc`, and `host`.
        #[builder(into, default)]
        pub network_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Process namespace to use for the containers in the task. The valid values are `host` and `task`.
        #[builder(into, default)]
        pub pid_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for rules that are taken into consideration during task placement. Maximum number of `placement_constraints` is `10`. Detailed below.
        #[builder(into, default)]
        pub placement_constraints: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ecs::TaskDefinitionPlacementConstraint>>,
        >,
        /// Configuration block for the App Mesh proxy. Detailed below.
        #[builder(into, default)]
        pub proxy_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ecs::TaskDefinitionProxyConfiguration>,
        >,
        /// Set of launch types required by the task. The valid values are `EC2` and `FARGATE`.
        #[builder(into, default)]
        pub requires_compatibilities: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Configuration block for runtime_platform that containers in your task may use.
        #[builder(into, default)]
        pub runtime_platform: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ecs::TaskDefinitionRuntimePlatform>,
        >,
        /// Whether to retain the old revision when the resource is destroyed or replacement is necessary. Default is `false`.
        #[builder(into, default)]
        pub skip_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ARN of IAM role that allows your Amazon ECS container task to make calls to other AWS services.
        #[builder(into, default)]
        pub task_role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether should track latest `ACTIVE` task definition on AWS or the one created with the resource stored in state. Default is `false`. Useful in the event the task definition is modified outside of this resource.
        #[builder(into, default)]
        pub track_latest: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Configuration block for volumes that containers in your task may use. Detailed below.
        #[builder(into, default)]
        pub volumes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ecs::TaskDefinitionVolume>>,
        >,
    }
    #[allow(dead_code)]
    pub struct TaskDefinitionResult {
        /// Full ARN of the Task Definition (including both `family` and `revision`).
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the Task Definition with the trailing `revision` removed. This may be useful for situations where the latest task definition is always desired. If a revision isn't specified, the latest ACTIVE revision is used. See the [AWS documentation](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_StartTask.html#ECS-StartTask-request-taskDefinition) for details.
        pub arn_without_revision: pulumi_gestalt_rust::Output<String>,
        /// A list of valid [container definitions](http://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_ContainerDefinition.html) provided as a single valid JSON document. Please note that you should only provide values that are part of the container definition document. For a detailed description of what parameters are available, see the [Task Definition Parameters](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_definition_parameters.html) section from the official [Developer Guide](https://docs.aws.amazon.com/AmazonECS/latest/developerguide).
        pub container_definitions: pulumi_gestalt_rust::Output<String>,
        /// Number of cpu units used by the task. If the `requires_compatibilities` is `FARGATE` this field is required.
        pub cpu: pulumi_gestalt_rust::Output<Option<String>>,
        /// The amount of ephemeral storage to allocate for the task. This parameter is used to expand the total amount of ephemeral storage available, beyond the default amount, for tasks hosted on AWS Fargate. See Ephemeral Storage.
        pub ephemeral_storage: pulumi_gestalt_rust::Output<
            Option<super::super::types::ecs::TaskDefinitionEphemeralStorage>,
        >,
        /// ARN of the task execution role that the Amazon ECS container agent and the Docker daemon can assume.
        pub execution_role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// A unique name for your task definition.
        ///
        /// The following arguments are optional:
        pub family: pulumi_gestalt_rust::Output<String>,
        /// Configuration block(s) with Inference Accelerators settings. Detailed below.
        pub inference_accelerators: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ecs::TaskDefinitionInferenceAccelerator>>,
        >,
        /// IPC resource namespace to be used for the containers in the task The valid values are `host`, `task`, and `none`.
        pub ipc_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// Amount (in MiB) of memory used by the task. If the `requires_compatibilities` is `FARGATE` this field is required.
        pub memory: pulumi_gestalt_rust::Output<Option<String>>,
        /// Docker networking mode to use for the containers in the task. Valid values are `none`, `bridge`, `awsvpc`, and `host`.
        pub network_mode: pulumi_gestalt_rust::Output<String>,
        /// Process namespace to use for the containers in the task. The valid values are `host` and `task`.
        pub pid_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block for rules that are taken into consideration during task placement. Maximum number of `placement_constraints` is `10`. Detailed below.
        pub placement_constraints: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ecs::TaskDefinitionPlacementConstraint>>,
        >,
        /// Configuration block for the App Mesh proxy. Detailed below.
        pub proxy_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::ecs::TaskDefinitionProxyConfiguration>,
        >,
        /// Set of launch types required by the task. The valid values are `EC2` and `FARGATE`.
        pub requires_compatibilities: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Revision of the task in a particular family.
        pub revision: pulumi_gestalt_rust::Output<i32>,
        /// Configuration block for runtime_platform that containers in your task may use.
        pub runtime_platform: pulumi_gestalt_rust::Output<
            Option<super::super::types::ecs::TaskDefinitionRuntimePlatform>,
        >,
        /// Whether to retain the old revision when the resource is destroyed or replacement is necessary. Default is `false`.
        pub skip_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// ARN of IAM role that allows your Amazon ECS container task to make calls to other AWS services.
        pub task_role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether should track latest `ACTIVE` task definition on AWS or the one created with the resource stored in state. Default is `false`. Useful in the event the task definition is modified outside of this resource.
        pub track_latest: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Configuration block for volumes that containers in your task may use. Detailed below.
        pub volumes: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ecs::TaskDefinitionVolume>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TaskDefinitionArgs,
    ) -> TaskDefinitionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let container_definitions_binding = args
            .container_definitions
            .get_output(context);
        let cpu_binding = args.cpu.get_output(context);
        let ephemeral_storage_binding = args.ephemeral_storage.get_output(context);
        let execution_role_arn_binding = args.execution_role_arn.get_output(context);
        let family_binding = args.family.get_output(context);
        let inference_accelerators_binding = args
            .inference_accelerators
            .get_output(context);
        let ipc_mode_binding = args.ipc_mode.get_output(context);
        let memory_binding = args.memory.get_output(context);
        let network_mode_binding = args.network_mode.get_output(context);
        let pid_mode_binding = args.pid_mode.get_output(context);
        let placement_constraints_binding = args
            .placement_constraints
            .get_output(context);
        let proxy_configuration_binding = args.proxy_configuration.get_output(context);
        let requires_compatibilities_binding = args
            .requires_compatibilities
            .get_output(context);
        let runtime_platform_binding = args.runtime_platform.get_output(context);
        let skip_destroy_binding = args.skip_destroy.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let task_role_arn_binding = args.task_role_arn.get_output(context);
        let track_latest_binding = args.track_latest.get_output(context);
        let volumes_binding = args.volumes.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ecs/taskDefinition:TaskDefinition".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerDefinitions".into(),
                    value: &container_definitions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cpu".into(),
                    value: &cpu_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ephemeralStorage".into(),
                    value: &ephemeral_storage_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "executionRoleArn".into(),
                    value: &execution_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "family".into(),
                    value: &family_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inferenceAccelerators".into(),
                    value: &inference_accelerators_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipcMode".into(),
                    value: &ipc_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "memory".into(),
                    value: &memory_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkMode".into(),
                    value: &network_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pidMode".into(),
                    value: &pid_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "placementConstraints".into(),
                    value: &placement_constraints_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "proxyConfiguration".into(),
                    value: &proxy_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requiresCompatibilities".into(),
                    value: &requires_compatibilities_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runtimePlatform".into(),
                    value: &runtime_platform_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipDestroy".into(),
                    value: &skip_destroy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "taskRoleArn".into(),
                    value: &task_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trackLatest".into(),
                    value: &track_latest_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "volumes".into(),
                    value: &volumes_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TaskDefinitionResult {
            arn: o.get_field("arn"),
            arn_without_revision: o.get_field("arnWithoutRevision"),
            container_definitions: o.get_field("containerDefinitions"),
            cpu: o.get_field("cpu"),
            ephemeral_storage: o.get_field("ephemeralStorage"),
            execution_role_arn: o.get_field("executionRoleArn"),
            family: o.get_field("family"),
            inference_accelerators: o.get_field("inferenceAccelerators"),
            ipc_mode: o.get_field("ipcMode"),
            memory: o.get_field("memory"),
            network_mode: o.get_field("networkMode"),
            pid_mode: o.get_field("pidMode"),
            placement_constraints: o.get_field("placementConstraints"),
            proxy_configuration: o.get_field("proxyConfiguration"),
            requires_compatibilities: o.get_field("requiresCompatibilities"),
            revision: o.get_field("revision"),
            runtime_platform: o.get_field("runtimePlatform"),
            skip_destroy: o.get_field("skipDestroy"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            task_role_arn: o.get_field("taskRoleArn"),
            track_latest: o.get_field("trackLatest"),
            volumes: o.get_field("volumes"),
        }
    }
}
