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
pub mod deployment_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeploymentConfigArgs {
        /// The compute platform can be `Server`, `Lambda`, or `ECS`. Default is `Server`.
        #[builder(into, default)]
        pub compute_platform: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the deployment config.
        #[builder(into, default)]
        pub deployment_config_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A minimum_healthy_hosts block. Required for `Server` compute platform. Minimum Healthy Hosts are documented below.
        #[builder(into, default)]
        pub minimum_healthy_hosts: pulumi_wasm_rust::Output<
            Option<super::super::types::codedeploy::DeploymentConfigMinimumHealthyHosts>,
        >,
        /// A traffic_routing_config block. Traffic Routing Config is documented below.
        #[builder(into, default)]
        pub traffic_routing_config: pulumi_wasm_rust::Output<
            Option<super::super::types::codedeploy::DeploymentConfigTrafficRoutingConfig>,
        >,
        /// A zonal_config block. Zonal Config is documented below.
        #[builder(into, default)]
        pub zonal_config: pulumi_wasm_rust::Output<
            Option<super::super::types::codedeploy::DeploymentConfigZonalConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct DeploymentConfigResult {
        /// The ARN of the deployment config.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The compute platform can be `Server`, `Lambda`, or `ECS`. Default is `Server`.
        pub compute_platform: pulumi_wasm_rust::Output<Option<String>>,
        /// The AWS Assigned deployment config id
        pub deployment_config_id: pulumi_wasm_rust::Output<String>,
        /// The name of the deployment config.
        pub deployment_config_name: pulumi_wasm_rust::Output<String>,
        /// A minimum_healthy_hosts block. Required for `Server` compute platform. Minimum Healthy Hosts are documented below.
        pub minimum_healthy_hosts: pulumi_wasm_rust::Output<
            Option<super::super::types::codedeploy::DeploymentConfigMinimumHealthyHosts>,
        >,
        /// A traffic_routing_config block. Traffic Routing Config is documented below.
        pub traffic_routing_config: pulumi_wasm_rust::Output<
            Option<super::super::types::codedeploy::DeploymentConfigTrafficRoutingConfig>,
        >,
        /// A zonal_config block. Zonal Config is documented below.
        pub zonal_config: pulumi_wasm_rust::Output<
            Option<super::super::types::codedeploy::DeploymentConfigZonalConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DeploymentConfigArgs) -> DeploymentConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let compute_platform_binding = args.compute_platform.get_inner();
        let deployment_config_name_binding = args.deployment_config_name.get_inner();
        let minimum_healthy_hosts_binding = args.minimum_healthy_hosts.get_inner();
        let traffic_routing_config_binding = args.traffic_routing_config.get_inner();
        let zonal_config_binding = args.zonal_config.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "computePlatform".into(),
                },
                register_interface::ResultField {
                    name: "deploymentConfigId".into(),
                },
                register_interface::ResultField {
                    name: "deploymentConfigName".into(),
                },
                register_interface::ResultField {
                    name: "minimumHealthyHosts".into(),
                },
                register_interface::ResultField {
                    name: "trafficRoutingConfig".into(),
                },
                register_interface::ResultField {
                    name: "zonalConfig".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DeploymentConfigResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            compute_platform: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("computePlatform").unwrap(),
            ),
            deployment_config_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deploymentConfigId").unwrap(),
            ),
            deployment_config_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deploymentConfigName").unwrap(),
            ),
            minimum_healthy_hosts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minimumHealthyHosts").unwrap(),
            ),
            traffic_routing_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trafficRoutingConfig").unwrap(),
            ),
            zonal_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zonalConfig").unwrap(),
            ),
        }
    }
}
