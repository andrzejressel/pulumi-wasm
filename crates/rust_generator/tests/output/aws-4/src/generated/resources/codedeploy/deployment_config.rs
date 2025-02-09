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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DeploymentConfigArgs,
    ) -> DeploymentConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let compute_platform_binding_1 = args.compute_platform.get_output(context);
        let compute_platform_binding = compute_platform_binding_1.get_inner();
        let deployment_config_name_binding_1 = args
            .deployment_config_name
            .get_output(context);
        let deployment_config_name_binding = deployment_config_name_binding_1
            .get_inner();
        let minimum_healthy_hosts_binding_1 = args
            .minimum_healthy_hosts
            .get_output(context);
        let minimum_healthy_hosts_binding = minimum_healthy_hosts_binding_1.get_inner();
        let traffic_routing_config_binding_1 = args
            .traffic_routing_config
            .get_output(context);
        let traffic_routing_config_binding = traffic_routing_config_binding_1
            .get_inner();
        let zonal_config_binding_1 = args.zonal_config.get_output(context);
        let zonal_config_binding = zonal_config_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codedeploy/deploymentConfig:DeploymentConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "computePlatform".into(),
                    value: &compute_platform_binding,
                },
                register_interface::ObjectField {
                    name: "deploymentConfigName".into(),
                    value: &deployment_config_name_binding,
                },
                register_interface::ObjectField {
                    name: "minimumHealthyHosts".into(),
                    value: &minimum_healthy_hosts_binding,
                },
                register_interface::ObjectField {
                    name: "trafficRoutingConfig".into(),
                    value: &traffic_routing_config_binding,
                },
                register_interface::ObjectField {
                    name: "zonalConfig".into(),
                    value: &zonal_config_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DeploymentConfigResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            compute_platform: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("computePlatform"),
            ),
            deployment_config_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deploymentConfigId"),
            ),
            deployment_config_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deploymentConfigName"),
            ),
            minimum_healthy_hosts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minimumHealthyHosts"),
            ),
            traffic_routing_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("trafficRoutingConfig"),
            ),
            zonal_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zonalConfig"),
            ),
        }
    }
}
