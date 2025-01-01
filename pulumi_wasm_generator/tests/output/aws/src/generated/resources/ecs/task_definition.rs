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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod task_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TaskDefinitionArgs {
        /// A list of valid [container definitions](http://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_ContainerDefinition.html) provided as a single valid JSON document. Please note that you should only provide values that are part of the container definition document. For a detailed description of what parameters are available, see the [Task Definition Parameters](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_definition_parameters.html) section from the official [Developer Guide](https://docs.aws.amazon.com/AmazonECS/latest/developerguide).
        #[builder(into)]
        pub container_definitions: pulumi_wasm_rust::Output<String>,
        /// Number of cpu units used by the task. If the `requires_compatibilities` is `FARGATE` this field is required.
        #[builder(into, default)]
        pub cpu: pulumi_wasm_rust::Output<Option<String>>,
        /// The amount of ephemeral storage to allocate for the task. This parameter is used to expand the total amount of ephemeral storage available, beyond the default amount, for tasks hosted on AWS Fargate. See Ephemeral Storage.
        #[builder(into, default)]
        pub ephemeral_storage: pulumi_wasm_rust::Output<
            Option<super::super::types::ecs::TaskDefinitionEphemeralStorage>,
        >,
        /// ARN of the task execution role that the Amazon ECS container agent and the Docker daemon can assume.
        #[builder(into, default)]
        pub execution_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// A unique name for your task definition.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub family: pulumi_wasm_rust::Output<String>,
        /// Configuration block(s) with Inference Accelerators settings. Detailed below.
        #[builder(into, default)]
        pub inference_accelerators: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ecs::TaskDefinitionInferenceAccelerator>>,
        >,
        /// IPC resource namespace to be used for the containers in the task The valid values are `host`, `task`, and `none`.
        #[builder(into, default)]
        pub ipc_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Amount (in MiB) of memory used by the task. If the `requires_compatibilities` is `FARGATE` this field is required.
        #[builder(into, default)]
        pub memory: pulumi_wasm_rust::Output<Option<String>>,
        /// Docker networking mode to use for the containers in the task. Valid values are `none`, `bridge`, `awsvpc`, and `host`.
        #[builder(into, default)]
        pub network_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Process namespace to use for the containers in the task. The valid values are `host` and `task`.
        #[builder(into, default)]
        pub pid_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for rules that are taken into consideration during task placement. Maximum number of `placement_constraints` is `10`. Detailed below.
        #[builder(into, default)]
        pub placement_constraints: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ecs::TaskDefinitionPlacementConstraint>>,
        >,
        /// Configuration block for the App Mesh proxy. Detailed below.
        #[builder(into, default)]
        pub proxy_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::ecs::TaskDefinitionProxyConfiguration>,
        >,
        /// Set of launch types required by the task. The valid values are `EC2` and `FARGATE`.
        #[builder(into, default)]
        pub requires_compatibilities: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Configuration block for runtime_platform that containers in your task may use.
        #[builder(into, default)]
        pub runtime_platform: pulumi_wasm_rust::Output<
            Option<super::super::types::ecs::TaskDefinitionRuntimePlatform>,
        >,
        /// Whether to retain the old revision when the resource is destroyed or replacement is necessary. Default is `false`.
        #[builder(into, default)]
        pub skip_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ARN of IAM role that allows your Amazon ECS container task to make calls to other AWS services.
        #[builder(into, default)]
        pub task_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether should track latest `ACTIVE` task definition on AWS or the one created with the resource stored in state. Default is `false`. Useful in the event the task definition is modified outside of this resource.
        #[builder(into, default)]
        pub track_latest: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration block for volumes that containers in your task may use. Detailed below.
        #[builder(into, default)]
        pub volumes: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ecs::TaskDefinitionVolume>>,
        >,
    }
    #[allow(dead_code)]
    pub struct TaskDefinitionResult {
        /// Full ARN of the Task Definition (including both `family` and `revision`).
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ARN of the Task Definition with the trailing `revision` removed. This may be useful for situations where the latest task definition is always desired. If a revision isn't specified, the latest ACTIVE revision is used. See the [AWS documentation](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_StartTask.html#ECS-StartTask-request-taskDefinition) for details.
        pub arn_without_revision: pulumi_wasm_rust::Output<String>,
        /// A list of valid [container definitions](http://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_ContainerDefinition.html) provided as a single valid JSON document. Please note that you should only provide values that are part of the container definition document. For a detailed description of what parameters are available, see the [Task Definition Parameters](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_definition_parameters.html) section from the official [Developer Guide](https://docs.aws.amazon.com/AmazonECS/latest/developerguide).
        pub container_definitions: pulumi_wasm_rust::Output<String>,
        /// Number of cpu units used by the task. If the `requires_compatibilities` is `FARGATE` this field is required.
        pub cpu: pulumi_wasm_rust::Output<Option<String>>,
        /// The amount of ephemeral storage to allocate for the task. This parameter is used to expand the total amount of ephemeral storage available, beyond the default amount, for tasks hosted on AWS Fargate. See Ephemeral Storage.
        pub ephemeral_storage: pulumi_wasm_rust::Output<
            Option<super::super::types::ecs::TaskDefinitionEphemeralStorage>,
        >,
        /// ARN of the task execution role that the Amazon ECS container agent and the Docker daemon can assume.
        pub execution_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// A unique name for your task definition.
        ///
        /// The following arguments are optional:
        pub family: pulumi_wasm_rust::Output<String>,
        /// Configuration block(s) with Inference Accelerators settings. Detailed below.
        pub inference_accelerators: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ecs::TaskDefinitionInferenceAccelerator>>,
        >,
        /// IPC resource namespace to be used for the containers in the task The valid values are `host`, `task`, and `none`.
        pub ipc_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Amount (in MiB) of memory used by the task. If the `requires_compatibilities` is `FARGATE` this field is required.
        pub memory: pulumi_wasm_rust::Output<Option<String>>,
        /// Docker networking mode to use for the containers in the task. Valid values are `none`, `bridge`, `awsvpc`, and `host`.
        pub network_mode: pulumi_wasm_rust::Output<String>,
        /// Process namespace to use for the containers in the task. The valid values are `host` and `task`.
        pub pid_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for rules that are taken into consideration during task placement. Maximum number of `placement_constraints` is `10`. Detailed below.
        pub placement_constraints: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ecs::TaskDefinitionPlacementConstraint>>,
        >,
        /// Configuration block for the App Mesh proxy. Detailed below.
        pub proxy_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::ecs::TaskDefinitionProxyConfiguration>,
        >,
        /// Set of launch types required by the task. The valid values are `EC2` and `FARGATE`.
        pub requires_compatibilities: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Revision of the task in a particular family.
        pub revision: pulumi_wasm_rust::Output<i32>,
        /// Configuration block for runtime_platform that containers in your task may use.
        pub runtime_platform: pulumi_wasm_rust::Output<
            Option<super::super::types::ecs::TaskDefinitionRuntimePlatform>,
        >,
        /// Whether to retain the old revision when the resource is destroyed or replacement is necessary. Default is `false`.
        pub skip_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// ARN of IAM role that allows your Amazon ECS container task to make calls to other AWS services.
        pub task_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether should track latest `ACTIVE` task definition on AWS or the one created with the resource stored in state. Default is `false`. Useful in the event the task definition is modified outside of this resource.
        pub track_latest: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration block for volumes that containers in your task may use. Detailed below.
        pub volumes: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ecs::TaskDefinitionVolume>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TaskDefinitionArgs) -> TaskDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let container_definitions_binding = args.container_definitions.get_inner();
        let cpu_binding = args.cpu.get_inner();
        let ephemeral_storage_binding = args.ephemeral_storage.get_inner();
        let execution_role_arn_binding = args.execution_role_arn.get_inner();
        let family_binding = args.family.get_inner();
        let inference_accelerators_binding = args.inference_accelerators.get_inner();
        let ipc_mode_binding = args.ipc_mode.get_inner();
        let memory_binding = args.memory.get_inner();
        let network_mode_binding = args.network_mode.get_inner();
        let pid_mode_binding = args.pid_mode.get_inner();
        let placement_constraints_binding = args.placement_constraints.get_inner();
        let proxy_configuration_binding = args.proxy_configuration.get_inner();
        let requires_compatibilities_binding = args.requires_compatibilities.get_inner();
        let runtime_platform_binding = args.runtime_platform.get_inner();
        let skip_destroy_binding = args.skip_destroy.get_inner();
        let tags_binding = args.tags.get_inner();
        let task_role_arn_binding = args.task_role_arn.get_inner();
        let track_latest_binding = args.track_latest.get_inner();
        let volumes_binding = args.volumes.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ecs/taskDefinition:TaskDefinition".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "containerDefinitions".into(),
                    value: &container_definitions_binding,
                },
                register_interface::ObjectField {
                    name: "cpu".into(),
                    value: &cpu_binding,
                },
                register_interface::ObjectField {
                    name: "ephemeralStorage".into(),
                    value: &ephemeral_storage_binding,
                },
                register_interface::ObjectField {
                    name: "executionRoleArn".into(),
                    value: &execution_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "family".into(),
                    value: &family_binding,
                },
                register_interface::ObjectField {
                    name: "inferenceAccelerators".into(),
                    value: &inference_accelerators_binding,
                },
                register_interface::ObjectField {
                    name: "ipcMode".into(),
                    value: &ipc_mode_binding,
                },
                register_interface::ObjectField {
                    name: "memory".into(),
                    value: &memory_binding,
                },
                register_interface::ObjectField {
                    name: "networkMode".into(),
                    value: &network_mode_binding,
                },
                register_interface::ObjectField {
                    name: "pidMode".into(),
                    value: &pid_mode_binding,
                },
                register_interface::ObjectField {
                    name: "placementConstraints".into(),
                    value: &placement_constraints_binding,
                },
                register_interface::ObjectField {
                    name: "proxyConfiguration".into(),
                    value: &proxy_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "requiresCompatibilities".into(),
                    value: &requires_compatibilities_binding,
                },
                register_interface::ObjectField {
                    name: "runtimePlatform".into(),
                    value: &runtime_platform_binding,
                },
                register_interface::ObjectField {
                    name: "skipDestroy".into(),
                    value: &skip_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "taskRoleArn".into(),
                    value: &task_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "trackLatest".into(),
                    value: &track_latest_binding,
                },
                register_interface::ObjectField {
                    name: "volumes".into(),
                    value: &volumes_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "arnWithoutRevision".into(),
                },
                register_interface::ResultField {
                    name: "containerDefinitions".into(),
                },
                register_interface::ResultField {
                    name: "cpu".into(),
                },
                register_interface::ResultField {
                    name: "ephemeralStorage".into(),
                },
                register_interface::ResultField {
                    name: "executionRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "family".into(),
                },
                register_interface::ResultField {
                    name: "inferenceAccelerators".into(),
                },
                register_interface::ResultField {
                    name: "ipcMode".into(),
                },
                register_interface::ResultField {
                    name: "memory".into(),
                },
                register_interface::ResultField {
                    name: "networkMode".into(),
                },
                register_interface::ResultField {
                    name: "pidMode".into(),
                },
                register_interface::ResultField {
                    name: "placementConstraints".into(),
                },
                register_interface::ResultField {
                    name: "proxyConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "requiresCompatibilities".into(),
                },
                register_interface::ResultField {
                    name: "revision".into(),
                },
                register_interface::ResultField {
                    name: "runtimePlatform".into(),
                },
                register_interface::ResultField {
                    name: "skipDestroy".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "taskRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "trackLatest".into(),
                },
                register_interface::ResultField {
                    name: "volumes".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TaskDefinitionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            arn_without_revision: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arnWithoutRevision").unwrap(),
            ),
            container_definitions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerDefinitions").unwrap(),
            ),
            cpu: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cpu").unwrap(),
            ),
            ephemeral_storage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ephemeralStorage").unwrap(),
            ),
            execution_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("executionRoleArn").unwrap(),
            ),
            family: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("family").unwrap(),
            ),
            inference_accelerators: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inferenceAccelerators").unwrap(),
            ),
            ipc_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipcMode").unwrap(),
            ),
            memory: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("memory").unwrap(),
            ),
            network_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkMode").unwrap(),
            ),
            pid_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pidMode").unwrap(),
            ),
            placement_constraints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("placementConstraints").unwrap(),
            ),
            proxy_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proxyConfiguration").unwrap(),
            ),
            requires_compatibilities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requiresCompatibilities").unwrap(),
            ),
            revision: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("revision").unwrap(),
            ),
            runtime_platform: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runtimePlatform").unwrap(),
            ),
            skip_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skipDestroy").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            task_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("taskRoleArn").unwrap(),
            ),
            track_latest: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trackLatest").unwrap(),
            ),
            volumes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumes").unwrap(),
            ),
        }
    }
}
