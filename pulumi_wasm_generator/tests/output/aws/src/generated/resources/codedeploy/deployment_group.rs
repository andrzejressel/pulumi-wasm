/// Provides a CodeDeploy Deployment Group for a CodeDeploy Application
///
/// > **NOTE on blue/green deployments:** When using `green_fleet_provisioning_option` with the `COPY_AUTO_SCALING_GROUP` action, CodeDeploy will create a new ASG with a different name. This ASG is _not_ managed by this provider and will conflict with existing configuration and state. You may want to use a different approach to managing deployments that involve multiple ASG, such as `DISCOVER_EXISTING` with separate blue and green ASG.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let assumeRole = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["sts:AssumeRole",]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["codedeploy.amazonaws.com",]). type ("Service")
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let aWSCodeDeployRole = role_policy_attachment::create(
///         "aWSCodeDeployRole",
///         RolePolicyAttachmentArgs::builder()
///             .policy_arn("arn:aws:iam::aws:policy/service-role/AWSCodeDeployRole")
///             .role("${example.name}")
///             .build_struct(),
///     );
///     let example = role::create(
///         "example",
///         RoleArgs::builder()
///             .assume_role_policy("${assumeRole.json}")
///             .name("example-role")
///             .build_struct(),
///     );
///     let exampleApplication = application::create(
///         "exampleApplication",
///         ApplicationArgs::builder().name("example-app").build_struct(),
///     );
///     let exampleDeploymentGroup = deployment_group::create(
///         "exampleDeploymentGroup",
///         DeploymentGroupArgs::builder()
///             .alarm_configuration(
///                 DeploymentGroupAlarmConfiguration::builder()
///                     .alarms(vec!["my-alarm-name",])
///                     .enabled(true)
///                     .build_struct(),
///             )
///             .app_name("${exampleApplication.name}")
///             .auto_rollback_configuration(
///                 DeploymentGroupAutoRollbackConfiguration::builder()
///                     .enabled(true)
///                     .events(vec!["DEPLOYMENT_FAILURE",])
///                     .build_struct(),
///             )
///             .deployment_group_name("example-group")
///             .ec_2_tag_sets(
///                 vec![
///                     DeploymentGroupEc2TagSet::builder()
///                     .ec2TagFilters(vec![DeploymentGroupEc2TagSetEc2TagFilter::builder()
///                     .key("filterkey1"). type ("KEY_AND_VALUE").value("filtervalue")
///                     .build_struct(), DeploymentGroupEc2TagSetEc2TagFilter::builder()
///                     .key("filterkey2"). type ("KEY_AND_VALUE").value("filtervalue")
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .outdated_instances_strategy("UPDATE")
///             .service_role_arn("${example.arn}")
///             .trigger_configurations(
///                 vec![
///                     DeploymentGroupTriggerConfiguration::builder()
///                     .triggerEvents(vec!["DeploymentFailure",])
///                     .triggerName("example-trigger")
///                     .triggerTargetArn("${exampleTopic.arn}").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleTopic = topic::create(
///         "exampleTopic",
///         TopicArgs::builder().name("example-topic").build_struct(),
///     );
/// }
/// ```
///
/// ### Blue Green Deployments with ECS
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = application::create(
///         "example",
///         ApplicationArgs::builder().compute_platform("ECS").name("example").build_struct(),
///     );
///     let exampleDeploymentGroup = deployment_group::create(
///         "exampleDeploymentGroup",
///         DeploymentGroupArgs::builder()
///             .app_name("${example.name}")
///             .auto_rollback_configuration(
///                 DeploymentGroupAutoRollbackConfiguration::builder()
///                     .enabled(true)
///                     .events(vec!["DEPLOYMENT_FAILURE",])
///                     .build_struct(),
///             )
///             .blue_green_deployment_config(
///                 DeploymentGroupBlueGreenDeploymentConfig::builder()
///                     .deploymentReadyOption(
///                         DeploymentGroupBlueGreenDeploymentConfigDeploymentReadyOption::builder()
///                             .actionOnTimeout("CONTINUE_DEPLOYMENT")
///                             .build_struct(),
///                     )
///                     .terminateBlueInstancesOnDeploymentSuccess(
///                         DeploymentGroupBlueGreenDeploymentConfigTerminateBlueInstancesOnDeploymentSuccess::builder()
///                             .action("TERMINATE")
///                             .terminationWaitTimeInMinutes(5)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .deployment_config_name("CodeDeployDefault.ECSAllAtOnce")
///             .deployment_group_name("example")
///             .deployment_style(
///                 DeploymentGroupDeploymentStyle::builder()
///                     .deploymentOption("WITH_TRAFFIC_CONTROL")
///                     .deploymentType("BLUE_GREEN")
///                     .build_struct(),
///             )
///             .ecs_service(
///                 DeploymentGroupEcsService::builder()
///                     .clusterName("${exampleAwsEcsCluster.name}")
///                     .serviceName("${exampleAwsEcsService.name}")
///                     .build_struct(),
///             )
///             .load_balancer_info(
///                 DeploymentGroupLoadBalancerInfo::builder()
///                     .targetGroupPairInfo(
///                         DeploymentGroupLoadBalancerInfoTargetGroupPairInfo::builder()
///                             .prodTrafficRoute(
///                                 DeploymentGroupLoadBalancerInfoTargetGroupPairInfoProdTrafficRoute::builder()
///                                     .listenerArns(vec!["${exampleAwsLbListener.arn}",])
///                                     .build_struct(),
///                             )
///                             .targetGroups(
///                                 vec![
///                                     DeploymentGroupLoadBalancerInfoTargetGroupPairInfoTargetGroup::builder()
///                                     .name("${blue.name}").build_struct(),
///                                     DeploymentGroupLoadBalancerInfoTargetGroupPairInfoTargetGroup::builder()
///                                     .name("${green.name}").build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .service_role_arn("${exampleAwsIamRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Blue Green Deployments with Servers and Classic ELB
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = application::create(
///         "example",
///         ApplicationArgs::builder().name("example-app").build_struct(),
///     );
///     let exampleDeploymentGroup = deployment_group::create(
///         "exampleDeploymentGroup",
///         DeploymentGroupArgs::builder()
///             .app_name("${example.name}")
///             .blue_green_deployment_config(
///                 DeploymentGroupBlueGreenDeploymentConfig::builder()
///                     .deploymentReadyOption(
///                         DeploymentGroupBlueGreenDeploymentConfigDeploymentReadyOption::builder()
///                             .actionOnTimeout("STOP_DEPLOYMENT")
///                             .waitTimeInMinutes(60)
///                             .build_struct(),
///                     )
///                     .greenFleetProvisioningOption(
///                         DeploymentGroupBlueGreenDeploymentConfigGreenFleetProvisioningOption::builder()
///                             .action("DISCOVER_EXISTING")
///                             .build_struct(),
///                     )
///                     .terminateBlueInstancesOnDeploymentSuccess(
///                         DeploymentGroupBlueGreenDeploymentConfigTerminateBlueInstancesOnDeploymentSuccess::builder()
///                             .action("KEEP_ALIVE")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .deployment_group_name("example-group")
///             .deployment_style(
///                 DeploymentGroupDeploymentStyle::builder()
///                     .deploymentOption("WITH_TRAFFIC_CONTROL")
///                     .deploymentType("BLUE_GREEN")
///                     .build_struct(),
///             )
///             .load_balancer_info(
///                 DeploymentGroupLoadBalancerInfo::builder()
///                     .elbInfos(
///                         vec![
///                             DeploymentGroupLoadBalancerInfoElbInfo::builder()
///                             .name("${exampleAwsElb.name}").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .service_role_arn("${exampleAwsIamRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeDeploy Deployment Groups using `app_name`, a colon, and `deployment_group_name`. For example:
///
/// ```sh
/// $ pulumi import aws:codedeploy/deploymentGroup:DeploymentGroup example my-application:my-deployment-group
/// ```
pub mod deployment_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeploymentGroupArgs {
        /// Configuration block of alarms associated with the deployment group (documented below).
        #[builder(into, default)]
        pub alarm_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::codedeploy::DeploymentGroupAlarmConfiguration>,
        >,
        /// The name of the application.
        #[builder(into)]
        pub app_name: pulumi_wasm_rust::Output<String>,
        /// Configuration block of the automatic rollback configuration associated with the deployment group (documented below).
        #[builder(into, default)]
        pub auto_rollback_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::codedeploy::DeploymentGroupAutoRollbackConfiguration,
            >,
        >,
        /// Autoscaling groups associated with the deployment group.
        #[builder(into, default)]
        pub autoscaling_groups: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Configuration block of the blue/green deployment options for a deployment group (documented below).
        #[builder(into, default)]
        pub blue_green_deployment_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::codedeploy::DeploymentGroupBlueGreenDeploymentConfig,
            >,
        >,
        /// The name of the group's deployment config. The default is "CodeDeployDefault.OneAtATime".
        #[builder(into, default)]
        pub deployment_config_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the deployment group.
        #[builder(into)]
        pub deployment_group_name: pulumi_wasm_rust::Output<String>,
        /// Configuration block of the type of deployment, either in-place or blue/green, you want to run and whether to route deployment traffic behind a load balancer (documented below).
        #[builder(into, default)]
        pub deployment_style: pulumi_wasm_rust::Output<
            Option<super::super::types::codedeploy::DeploymentGroupDeploymentStyle>,
        >,
        /// Tag filters associated with the deployment group. See the AWS docs for details.
        #[builder(into, default)]
        pub ec2_tag_filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::codedeploy::DeploymentGroupEc2TagFilter>>,
        >,
        /// Configuration block(s) of Tag filters associated with the deployment group, which are also referred to as tag groups (documented below). See the AWS docs for details.
        #[builder(into, default)]
        pub ec2_tag_sets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::codedeploy::DeploymentGroupEc2TagSet>>,
        >,
        /// Configuration block(s) of the ECS services for a deployment group (documented below).
        #[builder(into, default)]
        pub ecs_service: pulumi_wasm_rust::Output<
            Option<super::super::types::codedeploy::DeploymentGroupEcsService>,
        >,
        /// Single configuration block of the load balancer to use in a blue/green deployment (documented below).
        #[builder(into, default)]
        pub load_balancer_info: pulumi_wasm_rust::Output<
            Option<super::super::types::codedeploy::DeploymentGroupLoadBalancerInfo>,
        >,
        /// On premise tag filters associated with the group. See the AWS docs for details.
        #[builder(into, default)]
        pub on_premises_instance_tag_filters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::codedeploy::DeploymentGroupOnPremisesInstanceTagFilter,
                >,
            >,
        >,
        /// Configuration block of Indicates what happens when new Amazon EC2 instances are launched mid-deployment and do not receive the deployed application revision. Valid values are `UPDATE` and `IGNORE`. Defaults to `UPDATE`.
        #[builder(into, default)]
        pub outdated_instances_strategy: pulumi_wasm_rust::Output<Option<String>>,
        /// The service role ARN that allows deployments.
        #[builder(into)]
        pub service_role_arn: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Indicates whether the deployment group was configured to have CodeDeploy install a termination hook into an Auto Scaling group.
        #[builder(into, default)]
        pub termination_hook_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration block(s) of the triggers for the deployment group (documented below).
        #[builder(into, default)]
        pub trigger_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::codedeploy::DeploymentGroupTriggerConfiguration>,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct DeploymentGroupResult {
        /// Configuration block of alarms associated with the deployment group (documented below).
        pub alarm_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::codedeploy::DeploymentGroupAlarmConfiguration>,
        >,
        /// The name of the application.
        pub app_name: pulumi_wasm_rust::Output<String>,
        /// The ARN of the CodeDeploy deployment group.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Configuration block of the automatic rollback configuration associated with the deployment group (documented below).
        pub auto_rollback_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::codedeploy::DeploymentGroupAutoRollbackConfiguration,
            >,
        >,
        /// Autoscaling groups associated with the deployment group.
        pub autoscaling_groups: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Configuration block of the blue/green deployment options for a deployment group (documented below).
        pub blue_green_deployment_config: pulumi_wasm_rust::Output<
            super::super::types::codedeploy::DeploymentGroupBlueGreenDeploymentConfig,
        >,
        /// The destination platform type for the deployment.
        pub compute_platform: pulumi_wasm_rust::Output<String>,
        /// The name of the group's deployment config. The default is "CodeDeployDefault.OneAtATime".
        pub deployment_config_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the CodeDeploy deployment group.
        pub deployment_group_id: pulumi_wasm_rust::Output<String>,
        /// The name of the deployment group.
        pub deployment_group_name: pulumi_wasm_rust::Output<String>,
        /// Configuration block of the type of deployment, either in-place or blue/green, you want to run and whether to route deployment traffic behind a load balancer (documented below).
        pub deployment_style: pulumi_wasm_rust::Output<
            Option<super::super::types::codedeploy::DeploymentGroupDeploymentStyle>,
        >,
        /// Tag filters associated with the deployment group. See the AWS docs for details.
        pub ec2_tag_filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::codedeploy::DeploymentGroupEc2TagFilter>>,
        >,
        /// Configuration block(s) of Tag filters associated with the deployment group, which are also referred to as tag groups (documented below). See the AWS docs for details.
        pub ec2_tag_sets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::codedeploy::DeploymentGroupEc2TagSet>>,
        >,
        /// Configuration block(s) of the ECS services for a deployment group (documented below).
        pub ecs_service: pulumi_wasm_rust::Output<
            Option<super::super::types::codedeploy::DeploymentGroupEcsService>,
        >,
        /// Single configuration block of the load balancer to use in a blue/green deployment (documented below).
        pub load_balancer_info: pulumi_wasm_rust::Output<
            Option<super::super::types::codedeploy::DeploymentGroupLoadBalancerInfo>,
        >,
        /// On premise tag filters associated with the group. See the AWS docs for details.
        pub on_premises_instance_tag_filters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::codedeploy::DeploymentGroupOnPremisesInstanceTagFilter,
                >,
            >,
        >,
        /// Configuration block of Indicates what happens when new Amazon EC2 instances are launched mid-deployment and do not receive the deployed application revision. Valid values are `UPDATE` and `IGNORE`. Defaults to `UPDATE`.
        pub outdated_instances_strategy: pulumi_wasm_rust::Output<Option<String>>,
        /// The service role ARN that allows deployments.
        pub service_role_arn: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Indicates whether the deployment group was configured to have CodeDeploy install a termination hook into an Auto Scaling group.
        pub termination_hook_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration block(s) of the triggers for the deployment group (documented below).
        pub trigger_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::codedeploy::DeploymentGroupTriggerConfiguration>,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DeploymentGroupArgs) -> DeploymentGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alarm_configuration_binding = args.alarm_configuration.get_inner();
        let app_name_binding = args.app_name.get_inner();
        let auto_rollback_configuration_binding = args
            .auto_rollback_configuration
            .get_inner();
        let autoscaling_groups_binding = args.autoscaling_groups.get_inner();
        let blue_green_deployment_config_binding = args
            .blue_green_deployment_config
            .get_inner();
        let deployment_config_name_binding = args.deployment_config_name.get_inner();
        let deployment_group_name_binding = args.deployment_group_name.get_inner();
        let deployment_style_binding = args.deployment_style.get_inner();
        let ec2_tag_filters_binding = args.ec2_tag_filters.get_inner();
        let ec2_tag_sets_binding = args.ec2_tag_sets.get_inner();
        let ecs_service_binding = args.ecs_service.get_inner();
        let load_balancer_info_binding = args.load_balancer_info.get_inner();
        let on_premises_instance_tag_filters_binding = args
            .on_premises_instance_tag_filters
            .get_inner();
        let outdated_instances_strategy_binding = args
            .outdated_instances_strategy
            .get_inner();
        let service_role_arn_binding = args.service_role_arn.get_inner();
        let tags_binding = args.tags.get_inner();
        let termination_hook_enabled_binding = args.termination_hook_enabled.get_inner();
        let trigger_configurations_binding = args.trigger_configurations.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codedeploy/deploymentGroup:DeploymentGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alarmConfiguration".into(),
                    value: &alarm_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "appName".into(),
                    value: &app_name_binding,
                },
                register_interface::ObjectField {
                    name: "autoRollbackConfiguration".into(),
                    value: &auto_rollback_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "autoscalingGroups".into(),
                    value: &autoscaling_groups_binding,
                },
                register_interface::ObjectField {
                    name: "blueGreenDeploymentConfig".into(),
                    value: &blue_green_deployment_config_binding,
                },
                register_interface::ObjectField {
                    name: "deploymentConfigName".into(),
                    value: &deployment_config_name_binding,
                },
                register_interface::ObjectField {
                    name: "deploymentGroupName".into(),
                    value: &deployment_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "deploymentStyle".into(),
                    value: &deployment_style_binding,
                },
                register_interface::ObjectField {
                    name: "ec2TagFilters".into(),
                    value: &ec2_tag_filters_binding,
                },
                register_interface::ObjectField {
                    name: "ec2TagSets".into(),
                    value: &ec2_tag_sets_binding,
                },
                register_interface::ObjectField {
                    name: "ecsService".into(),
                    value: &ecs_service_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancerInfo".into(),
                    value: &load_balancer_info_binding,
                },
                register_interface::ObjectField {
                    name: "onPremisesInstanceTagFilters".into(),
                    value: &on_premises_instance_tag_filters_binding,
                },
                register_interface::ObjectField {
                    name: "outdatedInstancesStrategy".into(),
                    value: &outdated_instances_strategy_binding,
                },
                register_interface::ObjectField {
                    name: "serviceRoleArn".into(),
                    value: &service_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "terminationHookEnabled".into(),
                    value: &termination_hook_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "triggerConfigurations".into(),
                    value: &trigger_configurations_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "alarmConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "appName".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "autoRollbackConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "autoscalingGroups".into(),
                },
                register_interface::ResultField {
                    name: "blueGreenDeploymentConfig".into(),
                },
                register_interface::ResultField {
                    name: "computePlatform".into(),
                },
                register_interface::ResultField {
                    name: "deploymentConfigName".into(),
                },
                register_interface::ResultField {
                    name: "deploymentGroupId".into(),
                },
                register_interface::ResultField {
                    name: "deploymentGroupName".into(),
                },
                register_interface::ResultField {
                    name: "deploymentStyle".into(),
                },
                register_interface::ResultField {
                    name: "ec2TagFilters".into(),
                },
                register_interface::ResultField {
                    name: "ec2TagSets".into(),
                },
                register_interface::ResultField {
                    name: "ecsService".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancerInfo".into(),
                },
                register_interface::ResultField {
                    name: "onPremisesInstanceTagFilters".into(),
                },
                register_interface::ResultField {
                    name: "outdatedInstancesStrategy".into(),
                },
                register_interface::ResultField {
                    name: "serviceRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "terminationHookEnabled".into(),
                },
                register_interface::ResultField {
                    name: "triggerConfigurations".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DeploymentGroupResult {
            alarm_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alarmConfiguration").unwrap(),
            ),
            app_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appName").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auto_rollback_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoRollbackConfiguration").unwrap(),
            ),
            autoscaling_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoscalingGroups").unwrap(),
            ),
            blue_green_deployment_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blueGreenDeploymentConfig").unwrap(),
            ),
            compute_platform: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("computePlatform").unwrap(),
            ),
            deployment_config_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deploymentConfigName").unwrap(),
            ),
            deployment_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deploymentGroupId").unwrap(),
            ),
            deployment_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deploymentGroupName").unwrap(),
            ),
            deployment_style: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deploymentStyle").unwrap(),
            ),
            ec2_tag_filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ec2TagFilters").unwrap(),
            ),
            ec2_tag_sets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ec2TagSets").unwrap(),
            ),
            ecs_service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ecsService").unwrap(),
            ),
            load_balancer_info: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancerInfo").unwrap(),
            ),
            on_premises_instance_tag_filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("onPremisesInstanceTagFilters").unwrap(),
            ),
            outdated_instances_strategy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outdatedInstancesStrategy").unwrap(),
            ),
            service_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceRoleArn").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            termination_hook_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("terminationHookEnabled").unwrap(),
            ),
            trigger_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("triggerConfigurations").unwrap(),
            ),
        }
    }
}