/// > **Note:** To prevent a race condition during service deletion, make sure to set `depends_on` to the related `aws.iam.RolePolicy`; otherwise, the policy may be destroyed too soon and the ECS service will then get stuck in the `DRAINING` state.
///
/// Provides an ECS service - effectively a task that is expected to run until an error occurs or a user terminates it (typically a webserver or a database).
///
/// See [ECS Services section in AWS developer guide](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs_services.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let mongo = service::create(
///         "mongo",
///         ServiceArgs::builder()
///             .cluster("${fooAwsEcsCluster.id}")
///             .desired_count(3)
///             .iam_role("${fooAwsIamRole.arn}")
///             .load_balancers(
///                 vec![
///                     ServiceLoadBalancer::builder().containerName("mongo")
///                     .containerPort(8080).targetGroupArn("${fooAwsLbTargetGroup.arn}")
///                     .build_struct(),
///                 ],
///             )
///             .name("mongodb")
///             .ordered_placement_strategies(
///                 vec![
///                     ServiceOrderedPlacementStrategy::builder().field("cpu"). type
///                     ("binpack").build_struct(),
///                 ],
///             )
///             .placement_constraints(
///                 vec![
///                     ServicePlacementConstraint::builder()
///                     .expression("attribute:ecs.availability-zone in [us-west-2a, us-west-2b]")
///                     . type ("memberOf").build_struct(),
///                 ],
///             )
///             .task_definition("${mongoAwsEcsTaskDefinition.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Ignoring Changes to Desired Count
///
/// You can use [`ignoreChanges`](https://www.pulumi.com/docs/intro/concepts/programming-model/#ignorechanges) to create an ECS service with an initial count of running instances, then ignore any changes to that count caused externally (e.g. Application Autoscaling).
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = service::create(
///         "example",
///         ServiceArgs::builder().desired_count(2).build_struct(),
///     );
/// }
/// ```
///
/// ### Daemon Scheduling Strategy
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let bar = service::create(
///         "bar",
///         ServiceArgs::builder()
///             .cluster("${foo.id}")
///             .name("bar")
///             .scheduling_strategy("DAEMON")
///             .task_definition("${barAwsEcsTaskDefinition.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### CloudWatch Deployment Alarms
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = service::create(
///         "example",
///         ServiceArgs::builder()
///             .alarms(
///                 ServiceAlarms::builder()
///                     .alarmNames(vec!["${exampleAwsCloudwatchMetricAlarm.alarmName}",])
///                     .enable(true)
///                     .rollback(true)
///                     .build_struct(),
///             )
///             .cluster("${exampleAwsEcsCluster.id}")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### External Deployment Controller
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ecs:Service
///     properties:
///       name: example
///       cluster: ${exampleAwsEcsCluster.id}
///       deploymentController:
///         type: EXTERNAL
/// ```
///
/// ### Redeploy Service On Every Apply
///
/// The key used with `triggers` is arbitrary.
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ecs:Service
///     properties:
///       forceNewDeployment: true
///       triggers:
///         redeployment: plantimestamp()
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ECS services using the `name` together with ecs cluster `name`. For example:
///
/// ```sh
/// $ pulumi import aws:ecs/service:Service imported cluster-name/service-name
/// ```
pub mod service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// Information about the CloudWatch alarms. See below.
        #[builder(into, default)]
        pub alarms: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ecs::ServiceAlarms>,
        >,
        /// ECS automatically redistributes tasks within a service across Availability Zones (AZs) to mitigate the risk of impaired application availability due to underlying infrastructure failures and task lifecycle activities. The valid values are `ENABLED` and `DISABLED`. Defaults to `DISABLED`.
        #[builder(into, default)]
        pub availability_zone_rebalancing: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Capacity provider strategies to use for the service. Can be one or more. These can be updated without destroying and recreating the service only if `force_new_deployment = true` and not changing from 0 `capacity_provider_strategy` blocks to greater than 0, or vice versa. See below. Conflicts with `launch_type`.
        #[builder(into, default)]
        pub capacity_provider_strategies: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::ecs::ServiceCapacityProviderStrategy>>,
        >,
        /// ARN of an ECS cluster.
        #[builder(into, default)]
        pub cluster: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration block for deployment circuit breaker. See below.
        #[builder(into, default)]
        pub deployment_circuit_breaker: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ecs::ServiceDeploymentCircuitBreaker>,
        >,
        /// Configuration block for deployment controller configuration. See below.
        #[builder(into, default)]
        pub deployment_controller: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ecs::ServiceDeploymentController>,
        >,
        /// Upper limit (as a percentage of the service's desiredCount) of the number of running tasks that can be running in a service during a deployment. Not valid when using the `DAEMON` scheduling strategy.
        #[builder(into, default)]
        pub deployment_maximum_percent: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Lower limit (as a percentage of the service's desiredCount) of the number of running tasks that must remain running and healthy in a service during a deployment.
        #[builder(into, default)]
        pub deployment_minimum_healthy_percent: pulumi_wasm_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Number of instances of the task definition to place and keep running. Defaults to 0. Do not specify if using the `DAEMON` scheduling strategy.
        #[builder(into, default)]
        pub desired_count: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Whether to enable Amazon ECS managed tags for the tasks within the service.
        #[builder(into, default)]
        pub enable_ecs_managed_tags: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Whether to enable Amazon ECS Exec for the tasks within the service.
        #[builder(into, default)]
        pub enable_execute_command: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Enable to delete a service even if it wasn't scaled down to zero tasks. It's only necessary to use this if the service uses the `REPLICA` scheduling strategy.
        #[builder(into, default)]
        pub force_delete: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Enable to force a new task deployment of the service. This can be used to update tasks to use a newer Docker image with same image/tag combination (e.g., `myimage:latest`), roll Fargate tasks onto a newer platform version, or immediately deploy `ordered_placement_strategy` and `placement_constraints` updates.
        /// When using the forceNewDeployment property you also need to configure the triggers property.
        #[builder(into, default)]
        pub force_new_deployment: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Seconds to ignore failing load balancer health checks on newly instantiated tasks to prevent premature shutdown, up to 2147483647. Only valid for services configured to use load balancers.
        #[builder(into, default)]
        pub health_check_grace_period_seconds: pulumi_wasm_rust::InputOrOutput<
            Option<i32>,
        >,
        /// ARN of the IAM role that allows Amazon ECS to make calls to your load balancer on your behalf. This parameter is required if you are using a load balancer with your service, but only if your task definition does not use the `awsvpc` network mode. If using `awsvpc` network mode, do not specify this role. If your account has already created the Amazon ECS service-linked role, that role is used by default for your service unless you specify a role here.
        #[builder(into, default)]
        pub iam_role: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Launch type on which to run your service. The valid values are `EC2`, `FARGATE`, and `EXTERNAL`. Defaults to `EC2`. Conflicts with `capacity_provider_strategy`.
        #[builder(into, default)]
        pub launch_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration block for load balancers. See below.
        #[builder(into, default)]
        pub load_balancers: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::ecs::ServiceLoadBalancer>>,
        >,
        /// Name of the service (up to 255 letters, numbers, hyphens, and underscores)
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Network configuration for the service. This parameter is required for task definitions that use the `awsvpc` network mode to receive their own Elastic Network Interface, and it is not supported for other network modes. See below.
        #[builder(into, default)]
        pub network_configuration: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ecs::ServiceNetworkConfiguration>,
        >,
        /// Service level strategy rules that are taken into consideration during task placement. List from top to bottom in order of precedence. Updates to this configuration will take effect next task deployment unless `force_new_deployment` is enabled. The maximum number of `ordered_placement_strategy` blocks is `5`. See below.
        #[builder(into, default)]
        pub ordered_placement_strategies: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::ecs::ServiceOrderedPlacementStrategy>>,
        >,
        /// Rules that are taken into consideration during task placement. Updates to this configuration will take effect next task deployment unless `force_new_deployment` is enabled. Maximum number of `placement_constraints` is `10`. See below.
        #[builder(into, default)]
        pub placement_constraints: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::ecs::ServicePlacementConstraint>>,
        >,
        /// Platform version on which to run your service. Only applicable for `launch_type` set to `FARGATE`. Defaults to `LATEST`. More information about Fargate platform versions can be found in the [AWS ECS User Guide](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html).
        #[builder(into, default)]
        pub platform_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether to propagate the tags from the task definition or the service to the tasks. The valid values are `SERVICE` and `TASK_DEFINITION`.
        #[builder(into, default)]
        pub propagate_tags: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Scheduling strategy to use for the service. The valid values are `REPLICA` and `DAEMON`. Defaults to `REPLICA`. Note that [*Tasks using the Fargate launch type or the `CODE_DEPLOY` or `EXTERNAL` deployment controller types don't support the `DAEMON` scheduling strategy*](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_CreateService.html).
        #[builder(into, default)]
        pub scheduling_strategy: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ECS Service Connect configuration for this service to discover and connect to services, and be discovered by, and connected from, other services within a namespace. See below.
        #[builder(into, default)]
        pub service_connect_configuration: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ecs::ServiceServiceConnectConfiguration>,
        >,
        /// Service discovery registries for the service. The maximum number of `service_registries` blocks is `1`. See below.
        #[builder(into, default)]
        pub service_registries: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ecs::ServiceServiceRegistries>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Family and revision (`family:revision`) or full ARN of the task definition that you want to run in your service. Required unless using the `EXTERNAL` deployment controller. If a revision is not specified, the latest `ACTIVE` revision is used.
        #[builder(into, default)]
        pub task_definition: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of arbitrary keys and values that, when changed, will trigger an in-place update (redeployment). Useful with `"plantimestamp()"`. When using the triggers property you also need to set the forceNewDeployment property to True.
        #[builder(into, default)]
        pub triggers: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration for a volume specified in the task definition as a volume that is configured at launch time. Currently, the only supported volume type is an Amazon EBS volume. See below.
        #[builder(into, default)]
        pub volume_configuration: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ecs::ServiceVolumeConfiguration>,
        >,
        /// The VPC Lattice configuration for your service that allows Lattice to connect, secure, and monitor your service across multiple accounts and VPCs. See below.
        #[builder(into, default)]
        pub vpc_lattice_configurations: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::ecs::ServiceVpcLatticeConfiguration>>,
        >,
        /// If `true`, this provider will wait for the service to reach a steady state (like [`aws ecs wait services-stable`](https://docs.aws.amazon.com/cli/latest/reference/ecs/wait/services-stable.html)) before continuing. Default `false`.
        #[builder(into, default)]
        pub wait_for_steady_state: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// Information about the CloudWatch alarms. See below.
        pub alarms: pulumi_wasm_rust::Output<
            Option<super::super::types::ecs::ServiceAlarms>,
        >,
        /// ECS automatically redistributes tasks within a service across Availability Zones (AZs) to mitigate the risk of impaired application availability due to underlying infrastructure failures and task lifecycle activities. The valid values are `ENABLED` and `DISABLED`. Defaults to `DISABLED`.
        pub availability_zone_rebalancing: pulumi_wasm_rust::Output<Option<String>>,
        /// Capacity provider strategies to use for the service. Can be one or more. These can be updated without destroying and recreating the service only if `force_new_deployment = true` and not changing from 0 `capacity_provider_strategy` blocks to greater than 0, or vice versa. See below. Conflicts with `launch_type`.
        pub capacity_provider_strategies: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ecs::ServiceCapacityProviderStrategy>>,
        >,
        /// ARN of an ECS cluster.
        pub cluster: pulumi_wasm_rust::Output<String>,
        /// Configuration block for deployment circuit breaker. See below.
        pub deployment_circuit_breaker: pulumi_wasm_rust::Output<
            Option<super::super::types::ecs::ServiceDeploymentCircuitBreaker>,
        >,
        /// Configuration block for deployment controller configuration. See below.
        pub deployment_controller: pulumi_wasm_rust::Output<
            Option<super::super::types::ecs::ServiceDeploymentController>,
        >,
        /// Upper limit (as a percentage of the service's desiredCount) of the number of running tasks that can be running in a service during a deployment. Not valid when using the `DAEMON` scheduling strategy.
        pub deployment_maximum_percent: pulumi_wasm_rust::Output<Option<i32>>,
        /// Lower limit (as a percentage of the service's desiredCount) of the number of running tasks that must remain running and healthy in a service during a deployment.
        pub deployment_minimum_healthy_percent: pulumi_wasm_rust::Output<Option<i32>>,
        /// Number of instances of the task definition to place and keep running. Defaults to 0. Do not specify if using the `DAEMON` scheduling strategy.
        pub desired_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// Whether to enable Amazon ECS managed tags for the tasks within the service.
        pub enable_ecs_managed_tags: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to enable Amazon ECS Exec for the tasks within the service.
        pub enable_execute_command: pulumi_wasm_rust::Output<Option<bool>>,
        /// Enable to delete a service even if it wasn't scaled down to zero tasks. It's only necessary to use this if the service uses the `REPLICA` scheduling strategy.
        pub force_delete: pulumi_wasm_rust::Output<Option<bool>>,
        /// Enable to force a new task deployment of the service. This can be used to update tasks to use a newer Docker image with same image/tag combination (e.g., `myimage:latest`), roll Fargate tasks onto a newer platform version, or immediately deploy `ordered_placement_strategy` and `placement_constraints` updates.
        /// When using the forceNewDeployment property you also need to configure the triggers property.
        pub force_new_deployment: pulumi_wasm_rust::Output<Option<bool>>,
        /// Seconds to ignore failing load balancer health checks on newly instantiated tasks to prevent premature shutdown, up to 2147483647. Only valid for services configured to use load balancers.
        pub health_check_grace_period_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// ARN of the IAM role that allows Amazon ECS to make calls to your load balancer on your behalf. This parameter is required if you are using a load balancer with your service, but only if your task definition does not use the `awsvpc` network mode. If using `awsvpc` network mode, do not specify this role. If your account has already created the Amazon ECS service-linked role, that role is used by default for your service unless you specify a role here.
        pub iam_role: pulumi_wasm_rust::Output<String>,
        /// Launch type on which to run your service. The valid values are `EC2`, `FARGATE`, and `EXTERNAL`. Defaults to `EC2`. Conflicts with `capacity_provider_strategy`.
        pub launch_type: pulumi_wasm_rust::Output<String>,
        /// Configuration block for load balancers. See below.
        pub load_balancers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ecs::ServiceLoadBalancer>>,
        >,
        /// Name of the service (up to 255 letters, numbers, hyphens, and underscores)
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// Network configuration for the service. This parameter is required for task definitions that use the `awsvpc` network mode to receive their own Elastic Network Interface, and it is not supported for other network modes. See below.
        pub network_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::ecs::ServiceNetworkConfiguration>,
        >,
        /// Service level strategy rules that are taken into consideration during task placement. List from top to bottom in order of precedence. Updates to this configuration will take effect next task deployment unless `force_new_deployment` is enabled. The maximum number of `ordered_placement_strategy` blocks is `5`. See below.
        pub ordered_placement_strategies: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ecs::ServiceOrderedPlacementStrategy>>,
        >,
        /// Rules that are taken into consideration during task placement. Updates to this configuration will take effect next task deployment unless `force_new_deployment` is enabled. Maximum number of `placement_constraints` is `10`. See below.
        pub placement_constraints: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ecs::ServicePlacementConstraint>>,
        >,
        /// Platform version on which to run your service. Only applicable for `launch_type` set to `FARGATE`. Defaults to `LATEST`. More information about Fargate platform versions can be found in the [AWS ECS User Guide](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html).
        pub platform_version: pulumi_wasm_rust::Output<String>,
        /// Whether to propagate the tags from the task definition or the service to the tasks. The valid values are `SERVICE` and `TASK_DEFINITION`.
        pub propagate_tags: pulumi_wasm_rust::Output<Option<String>>,
        /// Scheduling strategy to use for the service. The valid values are `REPLICA` and `DAEMON`. Defaults to `REPLICA`. Note that [*Tasks using the Fargate launch type or the `CODE_DEPLOY` or `EXTERNAL` deployment controller types don't support the `DAEMON` scheduling strategy*](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_CreateService.html).
        pub scheduling_strategy: pulumi_wasm_rust::Output<Option<String>>,
        /// ECS Service Connect configuration for this service to discover and connect to services, and be discovered by, and connected from, other services within a namespace. See below.
        pub service_connect_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::ecs::ServiceServiceConnectConfiguration>,
        >,
        /// Service discovery registries for the service. The maximum number of `service_registries` blocks is `1`. See below.
        pub service_registries: pulumi_wasm_rust::Output<
            Option<super::super::types::ecs::ServiceServiceRegistries>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Family and revision (`family:revision`) or full ARN of the task definition that you want to run in your service. Required unless using the `EXTERNAL` deployment controller. If a revision is not specified, the latest `ACTIVE` revision is used.
        pub task_definition: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of arbitrary keys and values that, when changed, will trigger an in-place update (redeployment). Useful with `"plantimestamp()"`. When using the triggers property you also need to set the forceNewDeployment property to True.
        pub triggers: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration for a volume specified in the task definition as a volume that is configured at launch time. Currently, the only supported volume type is an Amazon EBS volume. See below.
        pub volume_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::ecs::ServiceVolumeConfiguration>,
        >,
        /// The VPC Lattice configuration for your service that allows Lattice to connect, secure, and monitor your service across multiple accounts and VPCs. See below.
        pub vpc_lattice_configurations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ecs::ServiceVpcLatticeConfiguration>>,
        >,
        /// If `true`, this provider will wait for the service to reach a steady state (like [`aws ecs wait services-stable`](https://docs.aws.amazon.com/cli/latest/reference/ecs/wait/services-stable.html)) before continuing. Default `false`.
        pub wait_for_steady_state: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ServiceArgs,
    ) -> ServiceResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alarms_binding = args.alarms.get_output(context).get_inner();
        let availability_zone_rebalancing_binding = args
            .availability_zone_rebalancing
            .get_output(context)
            .get_inner();
        let capacity_provider_strategies_binding = args
            .capacity_provider_strategies
            .get_output(context)
            .get_inner();
        let cluster_binding = args.cluster.get_output(context).get_inner();
        let deployment_circuit_breaker_binding = args
            .deployment_circuit_breaker
            .get_output(context)
            .get_inner();
        let deployment_controller_binding = args
            .deployment_controller
            .get_output(context)
            .get_inner();
        let deployment_maximum_percent_binding = args
            .deployment_maximum_percent
            .get_output(context)
            .get_inner();
        let deployment_minimum_healthy_percent_binding = args
            .deployment_minimum_healthy_percent
            .get_output(context)
            .get_inner();
        let desired_count_binding = args.desired_count.get_output(context).get_inner();
        let enable_ecs_managed_tags_binding = args
            .enable_ecs_managed_tags
            .get_output(context)
            .get_inner();
        let enable_execute_command_binding = args
            .enable_execute_command
            .get_output(context)
            .get_inner();
        let force_delete_binding = args.force_delete.get_output(context).get_inner();
        let force_new_deployment_binding = args
            .force_new_deployment
            .get_output(context)
            .get_inner();
        let health_check_grace_period_seconds_binding = args
            .health_check_grace_period_seconds
            .get_output(context)
            .get_inner();
        let iam_role_binding = args.iam_role.get_output(context).get_inner();
        let launch_type_binding = args.launch_type.get_output(context).get_inner();
        let load_balancers_binding = args.load_balancers.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_configuration_binding = args
            .network_configuration
            .get_output(context)
            .get_inner();
        let ordered_placement_strategies_binding = args
            .ordered_placement_strategies
            .get_output(context)
            .get_inner();
        let placement_constraints_binding = args
            .placement_constraints
            .get_output(context)
            .get_inner();
        let platform_version_binding = args
            .platform_version
            .get_output(context)
            .get_inner();
        let propagate_tags_binding = args.propagate_tags.get_output(context).get_inner();
        let scheduling_strategy_binding = args
            .scheduling_strategy
            .get_output(context)
            .get_inner();
        let service_connect_configuration_binding = args
            .service_connect_configuration
            .get_output(context)
            .get_inner();
        let service_registries_binding = args
            .service_registries
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let task_definition_binding = args
            .task_definition
            .get_output(context)
            .get_inner();
        let triggers_binding = args.triggers.get_output(context).get_inner();
        let volume_configuration_binding = args
            .volume_configuration
            .get_output(context)
            .get_inner();
        let vpc_lattice_configurations_binding = args
            .vpc_lattice_configurations
            .get_output(context)
            .get_inner();
        let wait_for_steady_state_binding = args
            .wait_for_steady_state
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ecs/service:Service".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alarms".into(),
                    value: &alarms_binding,
                },
                register_interface::ObjectField {
                    name: "availabilityZoneRebalancing".into(),
                    value: &availability_zone_rebalancing_binding,
                },
                register_interface::ObjectField {
                    name: "capacityProviderStrategies".into(),
                    value: &capacity_provider_strategies_binding,
                },
                register_interface::ObjectField {
                    name: "cluster".into(),
                    value: &cluster_binding,
                },
                register_interface::ObjectField {
                    name: "deploymentCircuitBreaker".into(),
                    value: &deployment_circuit_breaker_binding,
                },
                register_interface::ObjectField {
                    name: "deploymentController".into(),
                    value: &deployment_controller_binding,
                },
                register_interface::ObjectField {
                    name: "deploymentMaximumPercent".into(),
                    value: &deployment_maximum_percent_binding,
                },
                register_interface::ObjectField {
                    name: "deploymentMinimumHealthyPercent".into(),
                    value: &deployment_minimum_healthy_percent_binding,
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
                    name: "forceDelete".into(),
                    value: &force_delete_binding,
                },
                register_interface::ObjectField {
                    name: "forceNewDeployment".into(),
                    value: &force_new_deployment_binding,
                },
                register_interface::ObjectField {
                    name: "healthCheckGracePeriodSeconds".into(),
                    value: &health_check_grace_period_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "iamRole".into(),
                    value: &iam_role_binding,
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
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkConfiguration".into(),
                    value: &network_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "orderedPlacementStrategies".into(),
                    value: &ordered_placement_strategies_binding,
                },
                register_interface::ObjectField {
                    name: "placementConstraints".into(),
                    value: &placement_constraints_binding,
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
                    name: "schedulingStrategy".into(),
                    value: &scheduling_strategy_binding,
                },
                register_interface::ObjectField {
                    name: "serviceConnectConfiguration".into(),
                    value: &service_connect_configuration_binding,
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
                    name: "triggers".into(),
                    value: &triggers_binding,
                },
                register_interface::ObjectField {
                    name: "volumeConfiguration".into(),
                    value: &volume_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "vpcLatticeConfigurations".into(),
                    value: &vpc_lattice_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "waitForSteadyState".into(),
                    value: &wait_for_steady_state_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServiceResult {
            alarms: pulumi_wasm_rust::__private::into_domain(o.extract_field("alarms")),
            availability_zone_rebalancing: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("availabilityZoneRebalancing"),
            ),
            capacity_provider_strategies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("capacityProviderStrategies"),
            ),
            cluster: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cluster"),
            ),
            deployment_circuit_breaker: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deploymentCircuitBreaker"),
            ),
            deployment_controller: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deploymentController"),
            ),
            deployment_maximum_percent: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deploymentMaximumPercent"),
            ),
            deployment_minimum_healthy_percent: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deploymentMinimumHealthyPercent"),
            ),
            desired_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("desiredCount"),
            ),
            enable_ecs_managed_tags: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enableEcsManagedTags"),
            ),
            enable_execute_command: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enableExecuteCommand"),
            ),
            force_delete: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("forceDelete"),
            ),
            force_new_deployment: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("forceNewDeployment"),
            ),
            health_check_grace_period_seconds: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("healthCheckGracePeriodSeconds"),
            ),
            iam_role: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("iamRole"),
            ),
            launch_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("launchType"),
            ),
            load_balancers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("loadBalancers"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            network_configuration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("networkConfiguration"),
            ),
            ordered_placement_strategies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("orderedPlacementStrategies"),
            ),
            placement_constraints: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("placementConstraints"),
            ),
            platform_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("platformVersion"),
            ),
            propagate_tags: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("propagateTags"),
            ),
            scheduling_strategy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("schedulingStrategy"),
            ),
            service_connect_configuration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceConnectConfiguration"),
            ),
            service_registries: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceRegistries"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            task_definition: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("taskDefinition"),
            ),
            triggers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("triggers"),
            ),
            volume_configuration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("volumeConfiguration"),
            ),
            vpc_lattice_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vpcLatticeConfigurations"),
            ),
            wait_for_steady_state: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("waitForSteadyState"),
            ),
        }
    }
}
