/// Provides an ECS task set - effectively a task that is expected to run until an error occurs or a user terminates it (typically a webserver or a database).
///
/// See [ECS Task Set section in AWS developer guide](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-type-external.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = task_set::create(
///         "example",
///         TaskSetArgs::builder()
///             .cluster("${exampleAwsEcsCluster.id}")
///             .load_balancers(
///                 vec![
///                     TaskSetLoadBalancer::builder().containerName("mongo")
///                     .containerPort(8080).targetGroupArn("${exampleAwsLbTargetGroup.arn}")
///                     .build_struct(),
///                 ],
///             )
///             .service("${exampleAwsEcsService.id}")
///             .task_definition("${exampleAwsEcsTaskDefinition.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Ignoring Changes to Scale
///
/// You can utilize the generic resource lifecycle configuration block with `ignore_changes` to create an ECS service with an initial count of running instances, then ignore any changes to that count caused externally (e.g. Application Autoscaling).
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = task_set::create(
///         "example",
///         TaskSetArgs::builder()
///             .scale(TaskSetScale::builder().value(50).build_struct())
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ECS Task Sets using the `task_set_id`, `service`, and `cluster` separated by commas (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:ecs/taskSet:TaskSet example ecs-svc/7177320696926227436,arn:aws:ecs:us-west-2:123456789101:service/example/example-1234567890,arn:aws:ecs:us-west-2:123456789101:cluster/example
/// ```
pub mod task_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TaskSetArgs {
        /// The capacity provider strategy to use for the service. Can be one or more.  Defined below.
        #[builder(into, default)]
        pub capacity_provider_strategies: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::ecs::TaskSetCapacityProviderStrategy>>,
        >,
        /// The short name or ARN of the cluster that hosts the service to create the task set in.
        #[builder(into)]
        pub cluster: pulumi_wasm_rust::InputOrOutput<String>,
        /// The external ID associated with the task set.
        #[builder(into, default)]
        pub external_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether to allow deleting the task set without waiting for scaling down to 0. You can force a task set to delete even if it's in the process of scaling a resource. Normally, the provider drains all the tasks before deleting the task set. This bypasses that behavior and potentially leaves resources dangling.
        #[builder(into, default)]
        pub force_delete: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The launch type on which to run your service. The valid values are `EC2`, `FARGATE`, and `EXTERNAL`. Defaults to `EC2`.
        #[builder(into, default)]
        pub launch_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Details on load balancers that are used with a task set. Detailed below.
        #[builder(into, default)]
        pub load_balancers: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::ecs::TaskSetLoadBalancer>>,
        >,
        /// The network configuration for the service. This parameter is required for task definitions that use the `awsvpc` network mode to receive their own Elastic Network Interface, and it is not supported for other network modes. Detailed below.
        #[builder(into, default)]
        pub network_configuration: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ecs::TaskSetNetworkConfiguration>,
        >,
        /// The platform version on which to run your service. Only applicable for `launch_type` set to `FARGATE`. Defaults to `LATEST`. More information about Fargate platform versions can be found in the [AWS ECS User Guide](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html).
        #[builder(into, default)]
        pub platform_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A floating-point percentage of the desired number of tasks to place and keep running in the task set. Detailed below.
        #[builder(into, default)]
        pub scale: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ecs::TaskSetScale>,
        >,
        /// The short name or ARN of the ECS service.
        #[builder(into)]
        pub service: pulumi_wasm_rust::InputOrOutput<String>,
        /// The service discovery registries for the service. The maximum number of `service_registries` blocks is `1`. Detailed below.
        #[builder(into, default)]
        pub service_registries: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ecs::TaskSetServiceRegistries>,
        >,
        /// A map of tags to assign to the file system. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level. If you have set `copy_tags_to_backups` to true, and you specify one or more tags, no existing file system tags are copied from the file system to the backup.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The family and revision (`family:revision`) or full ARN of the task definition that you want to run in your service.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub task_definition: pulumi_wasm_rust::InputOrOutput<String>,
        /// Whether the provider should wait until the task set has reached `STEADY_STATE`.
        #[builder(into, default)]
        pub wait_until_stable: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Wait timeout for task set to reach `STEADY_STATE`. Valid time units include `ns`, `us` (or `µs`), `ms`, `s`, `m`, and `h`. Default `10m`.
        #[builder(into, default)]
        pub wait_until_stable_timeout: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TaskSetResult {
        /// The Amazon Resource Name (ARN) that identifies the task set.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The capacity provider strategy to use for the service. Can be one or more.  Defined below.
        pub capacity_provider_strategies: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ecs::TaskSetCapacityProviderStrategy>>,
        >,
        /// The short name or ARN of the cluster that hosts the service to create the task set in.
        pub cluster: pulumi_wasm_rust::Output<String>,
        /// The external ID associated with the task set.
        pub external_id: pulumi_wasm_rust::Output<String>,
        /// Whether to allow deleting the task set without waiting for scaling down to 0. You can force a task set to delete even if it's in the process of scaling a resource. Normally, the provider drains all the tasks before deleting the task set. This bypasses that behavior and potentially leaves resources dangling.
        pub force_delete: pulumi_wasm_rust::Output<Option<bool>>,
        /// The launch type on which to run your service. The valid values are `EC2`, `FARGATE`, and `EXTERNAL`. Defaults to `EC2`.
        pub launch_type: pulumi_wasm_rust::Output<String>,
        /// Details on load balancers that are used with a task set. Detailed below.
        pub load_balancers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ecs::TaskSetLoadBalancer>>,
        >,
        /// The network configuration for the service. This parameter is required for task definitions that use the `awsvpc` network mode to receive their own Elastic Network Interface, and it is not supported for other network modes. Detailed below.
        pub network_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::ecs::TaskSetNetworkConfiguration>,
        >,
        /// The platform version on which to run your service. Only applicable for `launch_type` set to `FARGATE`. Defaults to `LATEST`. More information about Fargate platform versions can be found in the [AWS ECS User Guide](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html).
        pub platform_version: pulumi_wasm_rust::Output<String>,
        /// A floating-point percentage of the desired number of tasks to place and keep running in the task set. Detailed below.
        pub scale: pulumi_wasm_rust::Output<super::super::types::ecs::TaskSetScale>,
        /// The short name or ARN of the ECS service.
        pub service: pulumi_wasm_rust::Output<String>,
        /// The service discovery registries for the service. The maximum number of `service_registries` blocks is `1`. Detailed below.
        pub service_registries: pulumi_wasm_rust::Output<
            Option<super::super::types::ecs::TaskSetServiceRegistries>,
        >,
        /// The stability status. This indicates whether the task set has reached a steady state.
        pub stability_status: pulumi_wasm_rust::Output<String>,
        /// The status of the task set.
        pub status: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the file system. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level. If you have set `copy_tags_to_backups` to true, and you specify one or more tags, no existing file system tags are copied from the file system to the backup.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The family and revision (`family:revision`) or full ARN of the task definition that you want to run in your service.
        ///
        /// The following arguments are optional:
        pub task_definition: pulumi_wasm_rust::Output<String>,
        /// The ID of the task set.
        pub task_set_id: pulumi_wasm_rust::Output<String>,
        /// Whether the provider should wait until the task set has reached `STEADY_STATE`.
        pub wait_until_stable: pulumi_wasm_rust::Output<Option<bool>>,
        /// Wait timeout for task set to reach `STEADY_STATE`. Valid time units include `ns`, `us` (or `µs`), `ms`, `s`, `m`, and `h`. Default `10m`.
        pub wait_until_stable_timeout: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: TaskSetArgs,
    ) -> TaskSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let capacity_provider_strategies_binding = args
            .capacity_provider_strategies
            .get_output(context)
            .get_inner();
        let cluster_binding = args.cluster.get_output(context).get_inner();
        let external_id_binding = args.external_id.get_output(context).get_inner();
        let force_delete_binding = args.force_delete.get_output(context).get_inner();
        let launch_type_binding = args.launch_type.get_output(context).get_inner();
        let load_balancers_binding = args.load_balancers.get_output(context).get_inner();
        let network_configuration_binding = args
            .network_configuration
            .get_output(context)
            .get_inner();
        let platform_version_binding = args
            .platform_version
            .get_output(context)
            .get_inner();
        let scale_binding = args.scale.get_output(context).get_inner();
        let service_binding = args.service.get_output(context).get_inner();
        let service_registries_binding = args
            .service_registries
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let task_definition_binding = args
            .task_definition
            .get_output(context)
            .get_inner();
        let wait_until_stable_binding = args
            .wait_until_stable
            .get_output(context)
            .get_inner();
        let wait_until_stable_timeout_binding = args
            .wait_until_stable_timeout
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ecs/taskSet:TaskSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "capacityProviderStrategies".into(),
                    value: &capacity_provider_strategies_binding,
                },
                register_interface::ObjectField {
                    name: "cluster".into(),
                    value: &cluster_binding,
                },
                register_interface::ObjectField {
                    name: "externalId".into(),
                    value: &external_id_binding,
                },
                register_interface::ObjectField {
                    name: "forceDelete".into(),
                    value: &force_delete_binding,
                },
                register_interface::ObjectField {
                    name: "launchType".into(),
                    value: &launch_type_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancers".into(),
                    value: &load_balancers_binding,
                },
                register_interface::ObjectField {
                    name: "networkConfiguration".into(),
                    value: &network_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "platformVersion".into(),
                    value: &platform_version_binding,
                },
                register_interface::ObjectField {
                    name: "scale".into(),
                    value: &scale_binding,
                },
                register_interface::ObjectField {
                    name: "service".into(),
                    value: &service_binding,
                },
                register_interface::ObjectField {
                    name: "serviceRegistries".into(),
                    value: &service_registries_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "taskDefinition".into(),
                    value: &task_definition_binding,
                },
                register_interface::ObjectField {
                    name: "waitUntilStable".into(),
                    value: &wait_until_stable_binding,
                },
                register_interface::ObjectField {
                    name: "waitUntilStableTimeout".into(),
                    value: &wait_until_stable_timeout_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TaskSetResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            capacity_provider_strategies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("capacityProviderStrategies"),
            ),
            cluster: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cluster"),
            ),
            external_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("externalId"),
            ),
            force_delete: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("forceDelete"),
            ),
            launch_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("launchType"),
            ),
            load_balancers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("loadBalancers"),
            ),
            network_configuration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("networkConfiguration"),
            ),
            platform_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("platformVersion"),
            ),
            scale: pulumi_wasm_rust::__private::into_domain(o.extract_field("scale")),
            service: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("service"),
            ),
            service_registries: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceRegistries"),
            ),
            stability_status: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("stabilityStatus"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            task_definition: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("taskDefinition"),
            ),
            task_set_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("taskSetId"),
            ),
            wait_until_stable: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("waitUntilStable"),
            ),
            wait_until_stable_timeout: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("waitUntilStableTimeout"),
            ),
        }
    }
}
