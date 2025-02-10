/// Provides a CodeDeploy Deployment Group for a CodeDeploy Application
///
/// > **NOTE on blue/green deployments:** When using `green_fleet_provisioning_option` with the `COPY_AUTO_SCALING_GROUP` action, CodeDeploy will create a new ASG with a different name. This ASG is _not_ managed by this provider and will conflict with existing configuration and state. You may want to use a different approach to managing deployments that involve multiple ASG, such as `DISCOVER_EXISTING` with separate blue and green ASG.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:iam:Role
///     properties:
///       name: example-role
///       assumeRolePolicy: ${assumeRole.json}
///   aWSCodeDeployRole:
///     type: aws:iam:RolePolicyAttachment
///     name: AWSCodeDeployRole
///     properties:
///       policyArn: arn:aws:iam::aws:policy/service-role/AWSCodeDeployRole
///       role: ${example.name}
///   exampleApplication:
///     type: aws:codedeploy:Application
///     name: example
///     properties:
///       name: example-app
///   exampleTopic:
///     type: aws:sns:Topic
///     name: example
///     properties:
///       name: example-topic
///   exampleDeploymentGroup:
///     type: aws:codedeploy:DeploymentGroup
///     name: example
///     properties:
///       appName: ${exampleApplication.name}
///       deploymentGroupName: example-group
///       serviceRoleArn: ${example.arn}
///       ec2TagSets:
///         - ec2TagFilters:
///             - key: filterkey1
///               type: KEY_AND_VALUE
///               value: filtervalue
///             - key: filterkey2
///               type: KEY_AND_VALUE
///               value: filtervalue
///       triggerConfigurations:
///         - triggerEvents:
///             - DeploymentFailure
///           triggerName: example-trigger
///           triggerTargetArn: ${exampleTopic.arn}
///       autoRollbackConfiguration:
///         enabled: true
///         events:
///           - DEPLOYMENT_FAILURE
///       alarmConfiguration:
///         alarms:
///           - my-alarm-name
///         enabled: true
///       outdatedInstancesStrategy: UPDATE
/// variables:
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - codedeploy.amazonaws.com
///             actions:
///               - sts:AssumeRole
/// ```
///
/// ### Blue Green Deployments with ECS
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod deployment_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeploymentGroupArgs {
        /// Configuration block of alarms associated with the deployment group (documented below).
        #[builder(into, default)]
        pub alarm_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::codedeploy::DeploymentGroupAlarmConfiguration>,
        >,
        /// The name of the application.
        #[builder(into)]
        pub app_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block of the automatic rollback configuration associated with the deployment group (documented below).
        #[builder(into, default)]
        pub auto_rollback_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::codedeploy::DeploymentGroupAutoRollbackConfiguration,
            >,
        >,
        /// Autoscaling groups associated with the deployment group.
        #[builder(into, default)]
        pub autoscaling_groups: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Configuration block of the blue/green deployment options for a deployment group (documented below).
        #[builder(into, default)]
        pub blue_green_deployment_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::codedeploy::DeploymentGroupBlueGreenDeploymentConfig,
            >,
        >,
        /// The name of the group's deployment config. The default is "CodeDeployDefault.OneAtATime".
        #[builder(into, default)]
        pub deployment_config_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the deployment group.
        #[builder(into)]
        pub deployment_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block of the type of deployment, either in-place or blue/green, you want to run and whether to route deployment traffic behind a load balancer (documented below).
        #[builder(into, default)]
        pub deployment_style: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::codedeploy::DeploymentGroupDeploymentStyle>,
        >,
        /// Tag filters associated with the deployment group. See the AWS docs for details.
        #[builder(into, default)]
        pub ec2_tag_filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::codedeploy::DeploymentGroupEc2TagFilter>>,
        >,
        /// Configuration block(s) of Tag filters associated with the deployment group, which are also referred to as tag groups (documented below). See the AWS docs for details.
        #[builder(into, default)]
        pub ec2_tag_sets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::codedeploy::DeploymentGroupEc2TagSet>>,
        >,
        /// Configuration block(s) of the ECS services for a deployment group (documented below).
        #[builder(into, default)]
        pub ecs_service: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::codedeploy::DeploymentGroupEcsService>,
        >,
        /// Single configuration block of the load balancer to use in a blue/green deployment (documented below).
        #[builder(into, default)]
        pub load_balancer_info: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::codedeploy::DeploymentGroupLoadBalancerInfo>,
        >,
        /// On premise tag filters associated with the group. See the AWS docs for details.
        #[builder(into, default)]
        pub on_premises_instance_tag_filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::codedeploy::DeploymentGroupOnPremisesInstanceTagFilter,
                >,
            >,
        >,
        /// Configuration block of Indicates what happens when new Amazon EC2 instances are launched mid-deployment and do not receive the deployed application revision. Valid values are `UPDATE` and `IGNORE`. Defaults to `UPDATE`.
        #[builder(into, default)]
        pub outdated_instances_strategy: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The service role ARN that allows deployments.
        #[builder(into)]
        pub service_role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Indicates whether the deployment group was configured to have CodeDeploy install a termination hook into an Auto Scaling group.
        #[builder(into, default)]
        pub termination_hook_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Configuration block(s) of the triggers for the deployment group (documented below).
        #[builder(into, default)]
        pub trigger_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::codedeploy::DeploymentGroupTriggerConfiguration>,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct DeploymentGroupResult {
        /// Configuration block of alarms associated with the deployment group (documented below).
        pub alarm_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::codedeploy::DeploymentGroupAlarmConfiguration>,
        >,
        /// The name of the application.
        pub app_name: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the CodeDeploy deployment group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration block of the automatic rollback configuration associated with the deployment group (documented below).
        pub auto_rollback_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::codedeploy::DeploymentGroupAutoRollbackConfiguration,
            >,
        >,
        /// Autoscaling groups associated with the deployment group.
        pub autoscaling_groups: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Configuration block of the blue/green deployment options for a deployment group (documented below).
        pub blue_green_deployment_config: pulumi_gestalt_rust::Output<
            super::super::types::codedeploy::DeploymentGroupBlueGreenDeploymentConfig,
        >,
        /// The destination platform type for the deployment.
        pub compute_platform: pulumi_gestalt_rust::Output<String>,
        /// The name of the group's deployment config. The default is "CodeDeployDefault.OneAtATime".
        pub deployment_config_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the CodeDeploy deployment group.
        pub deployment_group_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the deployment group.
        pub deployment_group_name: pulumi_gestalt_rust::Output<String>,
        /// Configuration block of the type of deployment, either in-place or blue/green, you want to run and whether to route deployment traffic behind a load balancer (documented below).
        pub deployment_style: pulumi_gestalt_rust::Output<
            Option<super::super::types::codedeploy::DeploymentGroupDeploymentStyle>,
        >,
        /// Tag filters associated with the deployment group. See the AWS docs for details.
        pub ec2_tag_filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::codedeploy::DeploymentGroupEc2TagFilter>>,
        >,
        /// Configuration block(s) of Tag filters associated with the deployment group, which are also referred to as tag groups (documented below). See the AWS docs for details.
        pub ec2_tag_sets: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::codedeploy::DeploymentGroupEc2TagSet>>,
        >,
        /// Configuration block(s) of the ECS services for a deployment group (documented below).
        pub ecs_service: pulumi_gestalt_rust::Output<
            Option<super::super::types::codedeploy::DeploymentGroupEcsService>,
        >,
        /// Single configuration block of the load balancer to use in a blue/green deployment (documented below).
        pub load_balancer_info: pulumi_gestalt_rust::Output<
            Option<super::super::types::codedeploy::DeploymentGroupLoadBalancerInfo>,
        >,
        /// On premise tag filters associated with the group. See the AWS docs for details.
        pub on_premises_instance_tag_filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::codedeploy::DeploymentGroupOnPremisesInstanceTagFilter,
                >,
            >,
        >,
        /// Configuration block of Indicates what happens when new Amazon EC2 instances are launched mid-deployment and do not receive the deployed application revision. Valid values are `UPDATE` and `IGNORE`. Defaults to `UPDATE`.
        pub outdated_instances_strategy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The service role ARN that allows deployments.
        pub service_role_arn: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Indicates whether the deployment group was configured to have CodeDeploy install a termination hook into an Auto Scaling group.
        pub termination_hook_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Configuration block(s) of the triggers for the deployment group (documented below).
        pub trigger_configurations: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::codedeploy::DeploymentGroupTriggerConfiguration>,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DeploymentGroupArgs,
    ) -> DeploymentGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let alarm_configuration_binding = args.alarm_configuration.get_output(context);
        let app_name_binding = args.app_name.get_output(context);
        let auto_rollback_configuration_binding = args
            .auto_rollback_configuration
            .get_output(context);
        let autoscaling_groups_binding = args.autoscaling_groups.get_output(context);
        let blue_green_deployment_config_binding = args
            .blue_green_deployment_config
            .get_output(context);
        let deployment_config_name_binding = args
            .deployment_config_name
            .get_output(context);
        let deployment_group_name_binding = args
            .deployment_group_name
            .get_output(context);
        let deployment_style_binding = args.deployment_style.get_output(context);
        let ec2_tag_filters_binding = args.ec2_tag_filters.get_output(context);
        let ec2_tag_sets_binding = args.ec2_tag_sets.get_output(context);
        let ecs_service_binding = args.ecs_service.get_output(context);
        let load_balancer_info_binding = args.load_balancer_info.get_output(context);
        let on_premises_instance_tag_filters_binding = args
            .on_premises_instance_tag_filters
            .get_output(context);
        let outdated_instances_strategy_binding = args
            .outdated_instances_strategy
            .get_output(context);
        let service_role_arn_binding = args.service_role_arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let termination_hook_enabled_binding = args
            .termination_hook_enabled
            .get_output(context);
        let trigger_configurations_binding = args
            .trigger_configurations
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:codedeploy/deploymentGroup:DeploymentGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alarmConfiguration".into(),
                    value: alarm_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appName".into(),
                    value: app_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoRollbackConfiguration".into(),
                    value: auto_rollback_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoscalingGroups".into(),
                    value: autoscaling_groups_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blueGreenDeploymentConfig".into(),
                    value: blue_green_deployment_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deploymentConfigName".into(),
                    value: deployment_config_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deploymentGroupName".into(),
                    value: deployment_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deploymentStyle".into(),
                    value: deployment_style_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ec2TagFilters".into(),
                    value: ec2_tag_filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ec2TagSets".into(),
                    value: ec2_tag_sets_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ecsService".into(),
                    value: ecs_service_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadBalancerInfo".into(),
                    value: load_balancer_info_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "onPremisesInstanceTagFilters".into(),
                    value: on_premises_instance_tag_filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "outdatedInstancesStrategy".into(),
                    value: outdated_instances_strategy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceRoleArn".into(),
                    value: service_role_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "terminationHookEnabled".into(),
                    value: termination_hook_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "triggerConfigurations".into(),
                    value: trigger_configurations_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DeploymentGroupResult {
            alarm_configuration: o.get_field("alarmConfiguration"),
            app_name: o.get_field("appName"),
            arn: o.get_field("arn"),
            auto_rollback_configuration: o.get_field("autoRollbackConfiguration"),
            autoscaling_groups: o.get_field("autoscalingGroups"),
            blue_green_deployment_config: o.get_field("blueGreenDeploymentConfig"),
            compute_platform: o.get_field("computePlatform"),
            deployment_config_name: o.get_field("deploymentConfigName"),
            deployment_group_id: o.get_field("deploymentGroupId"),
            deployment_group_name: o.get_field("deploymentGroupName"),
            deployment_style: o.get_field("deploymentStyle"),
            ec2_tag_filters: o.get_field("ec2TagFilters"),
            ec2_tag_sets: o.get_field("ec2TagSets"),
            ecs_service: o.get_field("ecsService"),
            load_balancer_info: o.get_field("loadBalancerInfo"),
            on_premises_instance_tag_filters: o
                .get_field("onPremisesInstanceTagFilters"),
            outdated_instances_strategy: o.get_field("outdatedInstancesStrategy"),
            service_role_arn: o.get_field("serviceRoleArn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            termination_hook_enabled: o.get_field("terminationHookEnabled"),
            trigger_configurations: o.get_field("triggerConfigurations"),
        }
    }
}
