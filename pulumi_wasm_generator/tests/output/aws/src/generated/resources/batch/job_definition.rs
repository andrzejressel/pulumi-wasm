/// Provides a Batch Job Definition resource.
///
/// ## Example Usage
///
/// ### Job definition of type container
///
/// ```yaml
/// resources:
///   test:
///     type: aws:batch:JobDefinition
///     properties:
///       name: my_test_batch_job_definition
///       type: container
///       containerProperties:
///         fn::toJSON:
///           command:
///             - ls
///             - -la
///           image: busybox
///           resourceRequirements:
///             - type: VCPU
///               value: '0.25'
///             - type: MEMORY
///               value: '512'
///           volumes:
///             - host:
///                 sourcePath: /tmp
///               name: tmp
///           environment:
///             - name: VARNAME
///               value: VARVAL
///           mountPoints:
///             - sourceVolume: tmp
///               containerPath: /tmp
///               readOnly: false
///           ulimits:
///             - hardLimit: 1024
///               name: nofile
///               softLimit: 1024
/// ```
///
/// ### Job definition of type multinode
///
/// ```yaml
/// resources:
///   test:
///     type: aws:batch:JobDefinition
///     properties:
///       name: tf_test_batch_job_definition_multinode
///       type: multinode
///       nodeProperties:
///         fn::toJSON:
///           mainNode: 0
///           nodeRangeProperties:
///             - container:
///                 command:
///                   - ls
///                   - -la
///                 image: busybox
///                 memory: 128
///                 vcpus: 1
///               targetNodes: '0:'
///             - container:
///                 command:
///                   - echo
///                   - test
///                 image: busybox
///                 memory: 128
///                 vcpus: 1
///               targetNodes: '1:'
///           numNodes: 2
/// ```
///
/// ### Job Definition of type EKS
///
/// ```yaml
/// resources:
///   test:
///     type: aws:batch:JobDefinition
///     properties:
///       name: ' tf_test_batch_job_definition_eks'
///       type: container
///       eksProperties:
///         podProperties:
///           hostNetwork: true
///           containers:
///             - image: public.ecr.aws/amazonlinux/amazonlinux:1
///               commands:
///                 - sleep
///                 - '60'
///               resources:
///                 limits:
///                   cpu: '1'
///                   memory: 1024Mi
///           metadata:
///             labels:
///               environment: test
/// ```
///
/// ### Fargate Platform Capability
///
/// ```yaml
/// resources:
///   ecsTaskExecutionRole:
///     type: aws:iam:Role
///     name: ecs_task_execution_role
///     properties:
///       name: my_test_batch_exec_role
///       assumeRolePolicy: ${assumeRolePolicy.json}
///   ecsTaskExecutionRolePolicy:
///     type: aws:iam:RolePolicyAttachment
///     name: ecs_task_execution_role_policy
///     properties:
///       role: ${ecsTaskExecutionRole.name}
///       policyArn: arn:aws:iam::aws:policy/service-role/AmazonECSTaskExecutionRolePolicy
///   test:
///     type: aws:batch:JobDefinition
///     properties:
///       name: my_test_batch_job_definition
///       type: container
///       platformCapabilities:
///         - FARGATE
///       containerProperties:
///         fn::toJSON:
///           command:
///             - echo
///             - test
///           image: busybox
///           jobRoleArn: arn:aws:iam::123456789012:role/AWSBatchS3ReadOnly
///           fargatePlatformConfiguration:
///             platformVersion: LATEST
///           resourceRequirements:
///             - type: VCPU
///               value: '0.25'
///             - type: MEMORY
///               value: '512'
///           executionRoleArn: ${ecsTaskExecutionRole.arn}
/// variables:
///   assumeRolePolicy:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - actions:
///               - sts:AssumeRole
///             principals:
///               - type: Service
///                 identifiers:
///                   - ecs-tasks.amazonaws.com
/// ```
///
/// ### Job definition of type container using `ecs_properties`
///
/// ```yaml
/// resources:
///   test:
///     type: aws:batch:JobDefinition
///     properties:
///       name: my_test_batch_job_definition
///       type: container
///       platformCapabilities:
///         - FARGATE
///       ecsProperties:
///         fn::toJSON:
///           taskProperties:
///             - executionRoleArn: ${ecsTaskExecutionRole.arn}
///               containers:
///                 - image: public.ecr.aws/amazonlinux/amazonlinux:1
///                   command:
///                     - sleep
///                     - '60'
///                   dependsOn:
///                     - containerName: container_b
///                       condition: COMPLETE
///                   secrets:
///                     - name: TEST
///                       valueFrom: DUMMY
///                   environment:
///                     - name: test
///                       value: Environment Variable
///                   essential: true
///                   logConfiguration:
///                     logDriver: awslogs
///                     options:
///                       awslogs-group: tf_test_batch_job
///                       awslogs-region: us-west-2
///                       awslogs-stream-prefix: ecs
///                   name: container_a
///                   privileged: false
///                   readonlyRootFilesystem: false
///                   resourceRequirements:
///                     - value: '1.0'
///                       type: VCPU
///                     - value: '2048'
///                       type: MEMORY
///                 - image: public.ecr.aws/amazonlinux/amazonlinux:1
///                   command:
///                     - sleep
///                     - '360'
///                   name: container_b
///                   essential: false
///                   resourceRequirements:
///                     - value: '1.0'
///                       type: VCPU
///                     - value: '2048'
///                       type: MEMORY
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Batch Job Definition using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:batch/jobDefinition:JobDefinition test arn:aws:batch:us-east-1:123456789012:job-definition/sample
/// ```
pub mod job_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct JobDefinitionArgs {
        /// Valid [container properties](http://docs.aws.amazon.com/batch/latest/APIReference/API_RegisterJobDefinition.html) provided as a single valid JSON document. This parameter is only valid if the `type` parameter is `container`.
        #[builder(into, default)]
        pub container_properties: pulumi_wasm_rust::Output<Option<String>>,
        /// When updating a job definition a new revision is created. This parameter determines if the previous version is `deregistered` (`INACTIVE`) or left  `ACTIVE`. Defaults to `true`.
        #[builder(into, default)]
        pub deregister_on_new_revision: pulumi_wasm_rust::Output<Option<bool>>,
        /// Valid [ECS properties](http://docs.aws.amazon.com/batch/latest/APIReference/API_RegisterJobDefinition.html) provided as a single valid JSON document. This parameter is only valid if the `type` parameter is `container`.
        #[builder(into, default)]
        pub ecs_properties: pulumi_wasm_rust::Output<Option<String>>,
        /// Valid eks properties. This parameter is only valid if the `type` parameter is `container`.
        #[builder(into, default)]
        pub eks_properties: pulumi_wasm_rust::Output<
            Option<super::super::types::batch::JobDefinitionEksProperties>,
        >,
        /// Name of the job definition.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Valid [node properties](http://docs.aws.amazon.com/batch/latest/APIReference/API_RegisterJobDefinition.html) provided as a single valid JSON document. This parameter is required if the `type` parameter is `multinode`.
        #[builder(into, default)]
        pub node_properties: pulumi_wasm_rust::Output<Option<String>>,
        /// Parameter substitution placeholders to set in the job definition.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Platform capabilities required by the job definition. If no value is specified, it defaults to `EC2`. To run the job on Fargate resources, specify `FARGATE`.
        #[builder(into, default)]
        pub platform_capabilities: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Whether to propagate the tags from the job definition to the corresponding Amazon ECS task. Default is `false`.
        #[builder(into, default)]
        pub propagate_tags: pulumi_wasm_rust::Output<Option<bool>>,
        /// Retry strategy to use for failed jobs that are submitted with this job definition. Maximum number of `retry_strategy` is `1`.  Defined below.
        #[builder(into, default)]
        pub retry_strategy: pulumi_wasm_rust::Output<
            Option<super::super::types::batch::JobDefinitionRetryStrategy>,
        >,
        /// Scheduling priority of the job definition. This only affects jobs in job queues with a fair share policy. Jobs with a higher scheduling priority are scheduled before jobs with a lower scheduling priority. Allowed values `0` through `9999`.
        #[builder(into, default)]
        pub scheduling_priority: pulumi_wasm_rust::Output<Option<i32>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Timeout for jobs so that if a job runs longer, AWS Batch terminates the job. Maximum number of `timeout` is `1`. Defined below.
        #[builder(into, default)]
        pub timeout: pulumi_wasm_rust::Output<
            Option<super::super::types::batch::JobDefinitionTimeout>,
        >,
        /// Type of job definition. Must be `container` or `multinode`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct JobDefinitionResult {
        /// ARN of the job definition, includes revision (`:#`).
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ARN without the revision number.
        pub arn_prefix: pulumi_wasm_rust::Output<String>,
        /// Valid [container properties](http://docs.aws.amazon.com/batch/latest/APIReference/API_RegisterJobDefinition.html) provided as a single valid JSON document. This parameter is only valid if the `type` parameter is `container`.
        pub container_properties: pulumi_wasm_rust::Output<Option<String>>,
        /// When updating a job definition a new revision is created. This parameter determines if the previous version is `deregistered` (`INACTIVE`) or left  `ACTIVE`. Defaults to `true`.
        pub deregister_on_new_revision: pulumi_wasm_rust::Output<Option<bool>>,
        /// Valid [ECS properties](http://docs.aws.amazon.com/batch/latest/APIReference/API_RegisterJobDefinition.html) provided as a single valid JSON document. This parameter is only valid if the `type` parameter is `container`.
        pub ecs_properties: pulumi_wasm_rust::Output<Option<String>>,
        /// Valid eks properties. This parameter is only valid if the `type` parameter is `container`.
        pub eks_properties: pulumi_wasm_rust::Output<
            Option<super::super::types::batch::JobDefinitionEksProperties>,
        >,
        /// Name of the job definition.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Valid [node properties](http://docs.aws.amazon.com/batch/latest/APIReference/API_RegisterJobDefinition.html) provided as a single valid JSON document. This parameter is required if the `type` parameter is `multinode`.
        pub node_properties: pulumi_wasm_rust::Output<Option<String>>,
        /// Parameter substitution placeholders to set in the job definition.
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Platform capabilities required by the job definition. If no value is specified, it defaults to `EC2`. To run the job on Fargate resources, specify `FARGATE`.
        pub platform_capabilities: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Whether to propagate the tags from the job definition to the corresponding Amazon ECS task. Default is `false`.
        pub propagate_tags: pulumi_wasm_rust::Output<Option<bool>>,
        /// Retry strategy to use for failed jobs that are submitted with this job definition. Maximum number of `retry_strategy` is `1`.  Defined below.
        pub retry_strategy: pulumi_wasm_rust::Output<
            Option<super::super::types::batch::JobDefinitionRetryStrategy>,
        >,
        /// Revision of the job definition.
        pub revision: pulumi_wasm_rust::Output<i32>,
        /// Scheduling priority of the job definition. This only affects jobs in job queues with a fair share policy. Jobs with a higher scheduling priority are scheduled before jobs with a lower scheduling priority. Allowed values `0` through `9999`.
        pub scheduling_priority: pulumi_wasm_rust::Output<Option<i32>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Timeout for jobs so that if a job runs longer, AWS Batch terminates the job. Maximum number of `timeout` is `1`. Defined below.
        pub timeout: pulumi_wasm_rust::Output<
            Option<super::super::types::batch::JobDefinitionTimeout>,
        >,
        /// Type of job definition. Must be `container` or `multinode`.
        ///
        /// The following arguments are optional:
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: JobDefinitionArgs) -> JobDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let container_properties_binding = args.container_properties.get_inner();
        let deregister_on_new_revision_binding = args
            .deregister_on_new_revision
            .get_inner();
        let ecs_properties_binding = args.ecs_properties.get_inner();
        let eks_properties_binding = args.eks_properties.get_inner();
        let name_binding = args.name.get_inner();
        let node_properties_binding = args.node_properties.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let platform_capabilities_binding = args.platform_capabilities.get_inner();
        let propagate_tags_binding = args.propagate_tags.get_inner();
        let retry_strategy_binding = args.retry_strategy.get_inner();
        let scheduling_priority_binding = args.scheduling_priority.get_inner();
        let tags_binding = args.tags.get_inner();
        let timeout_binding = args.timeout.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:batch/jobDefinition:JobDefinition".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "containerProperties".into(),
                    value: &container_properties_binding,
                },
                register_interface::ObjectField {
                    name: "deregisterOnNewRevision".into(),
                    value: &deregister_on_new_revision_binding,
                },
                register_interface::ObjectField {
                    name: "ecsProperties".into(),
                    value: &ecs_properties_binding,
                },
                register_interface::ObjectField {
                    name: "eksProperties".into(),
                    value: &eks_properties_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nodeProperties".into(),
                    value: &node_properties_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "platformCapabilities".into(),
                    value: &platform_capabilities_binding,
                },
                register_interface::ObjectField {
                    name: "propagateTags".into(),
                    value: &propagate_tags_binding,
                },
                register_interface::ObjectField {
                    name: "retryStrategy".into(),
                    value: &retry_strategy_binding,
                },
                register_interface::ObjectField {
                    name: "schedulingPriority".into(),
                    value: &scheduling_priority_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeout".into(),
                    value: &timeout_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "arnPrefix".into(),
                },
                register_interface::ResultField {
                    name: "containerProperties".into(),
                },
                register_interface::ResultField {
                    name: "deregisterOnNewRevision".into(),
                },
                register_interface::ResultField {
                    name: "ecsProperties".into(),
                },
                register_interface::ResultField {
                    name: "eksProperties".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nodeProperties".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "platformCapabilities".into(),
                },
                register_interface::ResultField {
                    name: "propagateTags".into(),
                },
                register_interface::ResultField {
                    name: "retryStrategy".into(),
                },
                register_interface::ResultField {
                    name: "revision".into(),
                },
                register_interface::ResultField {
                    name: "schedulingPriority".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "timeout".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        JobDefinitionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            arn_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arnPrefix").unwrap(),
            ),
            container_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerProperties").unwrap(),
            ),
            deregister_on_new_revision: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deregisterOnNewRevision").unwrap(),
            ),
            ecs_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ecsProperties").unwrap(),
            ),
            eks_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eksProperties").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            node_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeProperties").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            platform_capabilities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platformCapabilities").unwrap(),
            ),
            propagate_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("propagateTags").unwrap(),
            ),
            retry_strategy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retryStrategy").unwrap(),
            ),
            revision: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("revision").unwrap(),
            ),
            scheduling_priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schedulingPriority").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            timeout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeout").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}