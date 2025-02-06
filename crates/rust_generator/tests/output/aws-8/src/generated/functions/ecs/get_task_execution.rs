pub mod get_task_execution {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTaskExecutionArgs {
        /// Set of capacity provider strategies to use for the cluster. See below.
        #[builder(into, default)]
        pub capacity_provider_strategies: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::ecs::GetTaskExecutionCapacityProviderStrategy,
                >,
            >,
        >,
        /// An identifier that you provide to ensure the idempotency of the request. It must be unique and is case sensitive. Up to 64 characters are allowed. The valid characters are characters in the range of 33-126, inclusive. For more information, see [Ensuring idempotency](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/ECS_Idempotency.html).
        #[builder(into, default)]
        pub client_token: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Short name or full Amazon Resource Name (ARN) of the cluster to run the task on.
        #[builder(into)]
        pub cluster: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Number of instantiations of the specified task to place on your cluster. You can specify up to 10 tasks for each call.
        #[builder(into, default)]
        pub desired_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies whether to enable Amazon ECS managed tags for the tasks within the service.
        #[builder(into, default)]
        pub enable_ecs_managed_tags: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether to enable Amazon ECS Exec for the tasks within the service.
        #[builder(into, default)]
        pub enable_execute_command: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name of the task group to associate with the task. The default value is the family name of the task definition.
        #[builder(into, default)]
        pub group: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Launch type on which to run your service. Valid values are `EC2`, `FARGATE`, and `EXTERNAL`.
        #[builder(into, default)]
        pub launch_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Network configuration for the service. This parameter is required for task definitions that use the `awsvpc` network mode to receive their own Elastic Network Interface, and it is not supported for other network modes. See below.
        #[builder(into, default)]
        pub network_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::super::types::ecs::GetTaskExecutionNetworkConfiguration>,
        >,
        /// A list of container overrides that specify the name of a container in the specified task definition and the overrides it should receive.
        #[builder(into, default)]
        pub overrides: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::super::types::ecs::GetTaskExecutionOverrides>,
        >,
        /// An array of placement constraint objects to use for the task. You can specify up to 10 constraints for each task. See below.
        #[builder(into, default)]
        pub placement_constraints: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::super::types::ecs::GetTaskExecutionPlacementConstraint>,
            >,
        >,
        /// The placement strategy objects to use for the task. You can specify a maximum of 5 strategy rules for each task. See below.
        #[builder(into, default)]
        pub placement_strategies: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::super::types::ecs::GetTaskExecutionPlacementStrategy>,
            >,
        >,
        /// The platform version the task uses. A platform version is only specified for tasks hosted on Fargate. If one isn't specified, the `LATEST` platform version is used.
        #[builder(into, default)]
        pub platform_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether to propagate the tags from the task definition to the task. If no value is specified, the tags aren't propagated. An error will be received if you specify the `SERVICE` option when running a task. Valid values are `TASK_DEFINITION` or `NONE`.
        #[builder(into, default)]
        pub propagate_tags: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The reference ID to use for the task.
        #[builder(into, default)]
        pub reference_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An optional tag specified when a task is started.
        #[builder(into, default)]
        pub started_by: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The `family` and `revision` (`family:revision`) or full ARN of the task definition to run. If a revision isn't specified, the latest `ACTIVE` revision is used.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub task_definition: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetTaskExecutionResult {
        pub capacity_provider_strategies: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ecs::GetTaskExecutionCapacityProviderStrategy,
                >,
            >,
        >,
        pub client_token: pulumi_gestalt_rust::Output<Option<String>>,
        pub cluster: pulumi_gestalt_rust::Output<String>,
        pub desired_count: pulumi_gestalt_rust::Output<Option<i32>>,
        pub enable_ecs_managed_tags: pulumi_gestalt_rust::Output<Option<bool>>,
        pub enable_execute_command: pulumi_gestalt_rust::Output<Option<bool>>,
        pub group: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub launch_type: pulumi_gestalt_rust::Output<Option<String>>,
        pub network_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::super::types::ecs::GetTaskExecutionNetworkConfiguration>,
        >,
        pub overrides: pulumi_gestalt_rust::Output<
            Option<super::super::super::types::ecs::GetTaskExecutionOverrides>,
        >,
        pub placement_constraints: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::super::types::ecs::GetTaskExecutionPlacementConstraint>,
            >,
        >,
        pub placement_strategies: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::super::types::ecs::GetTaskExecutionPlacementStrategy>,
            >,
        >,
        pub platform_version: pulumi_gestalt_rust::Output<Option<String>>,
        pub propagate_tags: pulumi_gestalt_rust::Output<Option<String>>,
        pub reference_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub started_by: pulumi_gestalt_rust::Output<Option<String>>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of the provisioned task ARNs.
        pub task_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        pub task_definition: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetTaskExecutionArgs,
    ) -> GetTaskExecutionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let capacity_provider_strategies_binding = args
            .capacity_provider_strategies
            .get_output(context)
            .get_inner();
        let client_token_binding = args.client_token.get_output(context).get_inner();
        let cluster_binding = args.cluster.get_output(context).get_inner();
        let desired_count_binding = args.desired_count.get_output(context).get_inner();
        let enable_ecs_managed_tags_binding = args
            .enable_ecs_managed_tags
            .get_output(context)
            .get_inner();
        let enable_execute_command_binding = args
            .enable_execute_command
            .get_output(context)
            .get_inner();
        let group_binding = args.group.get_output(context).get_inner();
        let launch_type_binding = args.launch_type.get_output(context).get_inner();
        let network_configuration_binding = args
            .network_configuration
            .get_output(context)
            .get_inner();
        let overrides_binding = args.overrides.get_output(context).get_inner();
        let placement_constraints_binding = args
            .placement_constraints
            .get_output(context)
            .get_inner();
        let placement_strategies_binding = args
            .placement_strategies
            .get_output(context)
            .get_inner();
        let platform_version_binding = args
            .platform_version
            .get_output(context)
            .get_inner();
        let propagate_tags_binding = args.propagate_tags.get_output(context).get_inner();
        let reference_id_binding = args.reference_id.get_output(context).get_inner();
        let started_by_binding = args.started_by.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let task_definition_binding = args
            .task_definition
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ecs/getTaskExecution:getTaskExecution".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "capacityProviderStrategies".into(),
                    value: &capacity_provider_strategies_binding,
                },
                register_interface::ObjectField {
                    name: "clientToken".into(),
                    value: &client_token_binding,
                },
                register_interface::ObjectField {
                    name: "cluster".into(),
                    value: &cluster_binding,
                },
                register_interface::ObjectField {
                    name: "desiredCount".into(),
                    value: &desired_count_binding,
                },
                register_interface::ObjectField {
                    name: "enableEcsManagedTags".into(),
                    value: &enable_ecs_managed_tags_binding,
                },
                register_interface::ObjectField {
                    name: "enableExecuteCommand".into(),
                    value: &enable_execute_command_binding,
                },
                register_interface::ObjectField {
                    name: "group".into(),
                    value: &group_binding,
                },
                register_interface::ObjectField {
                    name: "launchType".into(),
                    value: &launch_type_binding,
                },
                register_interface::ObjectField {
                    name: "networkConfiguration".into(),
                    value: &network_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "overrides".into(),
                    value: &overrides_binding,
                },
                register_interface::ObjectField {
                    name: "placementConstraints".into(),
                    value: &placement_constraints_binding,
                },
                register_interface::ObjectField {
                    name: "placementStrategies".into(),
                    value: &placement_strategies_binding,
                },
                register_interface::ObjectField {
                    name: "platformVersion".into(),
                    value: &platform_version_binding,
                },
                register_interface::ObjectField {
                    name: "propagateTags".into(),
                    value: &propagate_tags_binding,
                },
                register_interface::ObjectField {
                    name: "referenceId".into(),
                    value: &reference_id_binding,
                },
                register_interface::ObjectField {
                    name: "startedBy".into(),
                    value: &started_by_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "taskDefinition".into(),
                    value: &task_definition_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetTaskExecutionResult {
            capacity_provider_strategies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("capacityProviderStrategies"),
            ),
            client_token: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientToken"),
            ),
            cluster: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cluster"),
            ),
            desired_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("desiredCount"),
            ),
            enable_ecs_managed_tags: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableEcsManagedTags"),
            ),
            enable_execute_command: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableExecuteCommand"),
            ),
            group: pulumi_gestalt_rust::__private::into_domain(o.extract_field("group")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            launch_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("launchType"),
            ),
            network_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkConfiguration"),
            ),
            overrides: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("overrides"),
            ),
            placement_constraints: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("placementConstraints"),
            ),
            placement_strategies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("placementStrategies"),
            ),
            platform_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("platformVersion"),
            ),
            propagate_tags: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("propagateTags"),
            ),
            reference_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("referenceId"),
            ),
            started_by: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("startedBy"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            task_arns: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("taskArns"),
            ),
            task_definition: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("taskDefinition"),
            ),
        }
    }
}
