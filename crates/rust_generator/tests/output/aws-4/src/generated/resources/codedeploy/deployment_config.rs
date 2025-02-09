/// Provides a CodeDeploy deployment config for an application
///
/// ## Example Usage
///
/// ### Server Usage
///
/// ```yaml
/// resources:
///   foo:
///     type: aws:codedeploy:DeploymentConfig
///     properties:
///       deploymentConfigName: test-deployment-config
///       minimumHealthyHosts:
///         type: HOST_COUNT
///         value: 2
///   fooDeploymentGroup:
///     type: aws:codedeploy:DeploymentGroup
///     name: foo
///     properties:
///       appName: ${fooApp.name}
///       deploymentGroupName: bar
///       serviceRoleArn: ${fooRole.arn}
///       deploymentConfigName: ${foo.id}
///       ec2TagFilters:
///         - key: filterkey
///           type: KEY_AND_VALUE
///           value: filtervalue
///       triggerConfigurations:
///         - triggerEvents:
///             - DeploymentFailure
///           triggerName: foo-trigger
///           triggerTargetArn: foo-topic-arn
///       autoRollbackConfiguration:
///         enabled: true
///         events:
///           - DEPLOYMENT_FAILURE
///       alarmConfiguration:
///         alarms:
///           - my-alarm-name
///         enabled: true
/// ```
///
/// ### Lambda Usage
///
/// ```yaml
/// resources:
///   foo:
///     type: aws:codedeploy:DeploymentConfig
///     properties:
///       deploymentConfigName: test-deployment-config
///       computePlatform: Lambda
///       trafficRoutingConfig:
///         type: TimeBasedLinear
///         timeBasedLinear:
///           interval: 10
///           percentage: 10
///   fooDeploymentGroup:
///     type: aws:codedeploy:DeploymentGroup
///     name: foo
///     properties:
///       appName: ${fooApp.name}
///       deploymentGroupName: bar
///       serviceRoleArn: ${fooRole.arn}
///       deploymentConfigName: ${foo.id}
///       autoRollbackConfiguration:
///         enabled: true
///         events:
///           - DEPLOYMENT_STOP_ON_ALARM
///       alarmConfiguration:
///         alarms:
///           - my-alarm-name
///         enabled: true
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeDeploy Deployment Configurations using the `deployment_config_name`. For example:
///
/// ```sh
/// $ pulumi import aws:codedeploy/deploymentConfig:DeploymentConfig example my-deployment-config
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod deployment_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeploymentConfigArgs {
        /// The compute platform can be `Server`, `Lambda`, or `ECS`. Default is `Server`.
        #[builder(into, default)]
        pub compute_platform: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the deployment config.
        #[builder(into, default)]
        pub deployment_config_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A minimum_healthy_hosts block. Required for `Server` compute platform. Minimum Healthy Hosts are documented below.
        #[builder(into, default)]
        pub minimum_healthy_hosts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::codedeploy::DeploymentConfigMinimumHealthyHosts>,
        >,
        /// A traffic_routing_config block. Traffic Routing Config is documented below.
        #[builder(into, default)]
        pub traffic_routing_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::codedeploy::DeploymentConfigTrafficRoutingConfig>,
        >,
        /// A zonal_config block. Zonal Config is documented below.
        #[builder(into, default)]
        pub zonal_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::codedeploy::DeploymentConfigZonalConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct DeploymentConfigResult {
        /// The ARN of the deployment config.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The compute platform can be `Server`, `Lambda`, or `ECS`. Default is `Server`.
        pub compute_platform: pulumi_gestalt_rust::Output<Option<String>>,
        /// The AWS Assigned deployment config id
        pub deployment_config_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the deployment config.
        pub deployment_config_name: pulumi_gestalt_rust::Output<String>,
        /// A minimum_healthy_hosts block. Required for `Server` compute platform. Minimum Healthy Hosts are documented below.
        pub minimum_healthy_hosts: pulumi_gestalt_rust::Output<
            Option<super::super::types::codedeploy::DeploymentConfigMinimumHealthyHosts>,
        >,
        /// A traffic_routing_config block. Traffic Routing Config is documented below.
        pub traffic_routing_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::codedeploy::DeploymentConfigTrafficRoutingConfig>,
        >,
        /// A zonal_config block. Zonal Config is documented below.
        pub zonal_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::codedeploy::DeploymentConfigZonalConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DeploymentConfigArgs,
    ) -> DeploymentConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let compute_platform_binding = args.compute_platform.get_output(context);
        let deployment_config_name_binding = args
            .deployment_config_name
            .get_output(context);
        let minimum_healthy_hosts_binding = args
            .minimum_healthy_hosts
            .get_output(context);
        let traffic_routing_config_binding = args
            .traffic_routing_config
            .get_output(context);
        let zonal_config_binding = args.zonal_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:codedeploy/deploymentConfig:DeploymentConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "computePlatform".into(),
                    value: compute_platform_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deploymentConfigName".into(),
                    value: deployment_config_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minimumHealthyHosts".into(),
                    value: minimum_healthy_hosts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trafficRoutingConfig".into(),
                    value: traffic_routing_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zonalConfig".into(),
                    value: zonal_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DeploymentConfigResult {
            arn: o.get_field("arn"),
            compute_platform: o.get_field("computePlatform"),
            deployment_config_id: o.get_field("deploymentConfigId"),
            deployment_config_name: o.get_field("deploymentConfigName"),
            minimum_healthy_hosts: o.get_field("minimumHealthyHosts"),
            traffic_routing_config: o.get_field("trafficRoutingConfig"),
            zonal_config: o.get_field("zonalConfig"),
        }
    }
}
