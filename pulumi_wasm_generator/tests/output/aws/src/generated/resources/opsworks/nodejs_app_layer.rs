/// Provides an OpsWorks NodeJS application layer resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let app = nodejs_app_layer::create(
///         "app",
///         NodejsAppLayerArgs::builder().stack_id("${main.id}").build_struct(),
///     );
/// }
/// ```
pub mod nodejs_app_layer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NodejsAppLayerArgs {
        /// Whether to automatically assign an elastic IP address to the layer's instances.
        #[builder(into, default)]
        pub auto_assign_elastic_ips: pulumi_wasm_rust::Output<Option<bool>>,
        /// For stacks belonging to a VPC, whether to automatically assign a public IP address to each of the layer's instances.
        #[builder(into, default)]
        pub auto_assign_public_ips: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to enable auto-healing for the layer.
        #[builder(into, default)]
        pub auto_healing: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub cloudwatch_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::opsworks::NodejsAppLayerCloudwatchConfiguration>,
        >,
        #[builder(into, default)]
        pub custom_configure_recipes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        #[builder(into, default)]
        pub custom_deploy_recipes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ARN of an IAM profile that will be used for the layer's instances.
        #[builder(into, default)]
        pub custom_instance_profile_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Custom JSON attributes to apply to the layer.
        #[builder(into, default)]
        pub custom_json: pulumi_wasm_rust::Output<Option<String>>,
        /// Ids for a set of security groups to apply to the layer's instances.
        #[builder(into, default)]
        pub custom_security_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        #[builder(into, default)]
        pub custom_setup_recipes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        #[builder(into, default)]
        pub custom_shutdown_recipes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        #[builder(into, default)]
        pub custom_undeploy_recipes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Whether to enable Elastic Load Balancing connection draining.
        #[builder(into, default)]
        pub drain_elb_on_shutdown: pulumi_wasm_rust::Output<Option<bool>>,
        /// `ebs_volume` blocks, as described below, will each create an EBS volume and connect it to the layer's instances.
        #[builder(into, default)]
        pub ebs_volumes: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::opsworks::NodejsAppLayerEbsVolume>>,
        >,
        /// Name of an Elastic Load Balancer to attach to this layer
        #[builder(into, default)]
        pub elastic_load_balancer: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to install OS and package updates on each instance when it boots.
        #[builder(into, default)]
        pub install_updates_on_boot: pulumi_wasm_rust::Output<Option<bool>>,
        /// The time, in seconds, that OpsWorks will wait for Chef to complete after triggering the Shutdown event.
        #[builder(into, default)]
        pub instance_shutdown_timeout: pulumi_wasm_rust::Output<Option<i32>>,
        #[builder(into, default)]
        pub load_based_auto_scaling: pulumi_wasm_rust::Output<
            Option<super::super::types::opsworks::NodejsAppLayerLoadBasedAutoScaling>,
        >,
        /// A human-readable name for the layer.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The version of NodeJS to use. Defaults to "0.10.38".
        #[builder(into, default)]
        pub nodejs_version: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the stack the layer will belong to.
        #[builder(into)]
        pub stack_id: pulumi_wasm_rust::Output<String>,
        /// Names of a set of system packages to install on the layer's instances.
        #[builder(into, default)]
        pub system_packages: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// The following extra optional arguments, all lists of Chef recipe names, allow
        /// custom Chef recipes to be applied to layer instances at the five different
        /// lifecycle events, if custom cookbooks are enabled on the layer's stack:
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whether to use EBS-optimized instances.
        #[builder(into, default)]
        pub use_ebs_optimized_instances: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct NodejsAppLayerResult {
        /// The Amazon Resource Name(ARN) of the layer.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Whether to automatically assign an elastic IP address to the layer's instances.
        pub auto_assign_elastic_ips: pulumi_wasm_rust::Output<Option<bool>>,
        /// For stacks belonging to a VPC, whether to automatically assign a public IP address to each of the layer's instances.
        pub auto_assign_public_ips: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to enable auto-healing for the layer.
        pub auto_healing: pulumi_wasm_rust::Output<Option<bool>>,
        pub cloudwatch_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::opsworks::NodejsAppLayerCloudwatchConfiguration>,
        >,
        pub custom_configure_recipes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub custom_deploy_recipes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ARN of an IAM profile that will be used for the layer's instances.
        pub custom_instance_profile_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Custom JSON attributes to apply to the layer.
        pub custom_json: pulumi_wasm_rust::Output<Option<String>>,
        /// Ids for a set of security groups to apply to the layer's instances.
        pub custom_security_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub custom_setup_recipes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub custom_shutdown_recipes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub custom_undeploy_recipes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Whether to enable Elastic Load Balancing connection draining.
        pub drain_elb_on_shutdown: pulumi_wasm_rust::Output<Option<bool>>,
        /// `ebs_volume` blocks, as described below, will each create an EBS volume and connect it to the layer's instances.
        pub ebs_volumes: pulumi_wasm_rust::Output<
            Vec<super::super::types::opsworks::NodejsAppLayerEbsVolume>,
        >,
        /// Name of an Elastic Load Balancer to attach to this layer
        pub elastic_load_balancer: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to install OS and package updates on each instance when it boots.
        pub install_updates_on_boot: pulumi_wasm_rust::Output<Option<bool>>,
        /// The time, in seconds, that OpsWorks will wait for Chef to complete after triggering the Shutdown event.
        pub instance_shutdown_timeout: pulumi_wasm_rust::Output<Option<i32>>,
        pub load_based_auto_scaling: pulumi_wasm_rust::Output<
            super::super::types::opsworks::NodejsAppLayerLoadBasedAutoScaling,
        >,
        /// A human-readable name for the layer.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The version of NodeJS to use. Defaults to "0.10.38".
        pub nodejs_version: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the stack the layer will belong to.
        pub stack_id: pulumi_wasm_rust::Output<String>,
        /// Names of a set of system packages to install on the layer's instances.
        pub system_packages: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// The following extra optional arguments, all lists of Chef recipe names, allow
        /// custom Chef recipes to be applied to layer instances at the five different
        /// lifecycle events, if custom cookbooks are enabled on the layer's stack:
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whether to use EBS-optimized instances.
        pub use_ebs_optimized_instances: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: NodejsAppLayerArgs) -> NodejsAppLayerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_assign_elastic_ips_binding = args.auto_assign_elastic_ips.get_inner();
        let auto_assign_public_ips_binding = args.auto_assign_public_ips.get_inner();
        let auto_healing_binding = args.auto_healing.get_inner();
        let cloudwatch_configuration_binding = args.cloudwatch_configuration.get_inner();
        let custom_configure_recipes_binding = args.custom_configure_recipes.get_inner();
        let custom_deploy_recipes_binding = args.custom_deploy_recipes.get_inner();
        let custom_instance_profile_arn_binding = args
            .custom_instance_profile_arn
            .get_inner();
        let custom_json_binding = args.custom_json.get_inner();
        let custom_security_group_ids_binding = args
            .custom_security_group_ids
            .get_inner();
        let custom_setup_recipes_binding = args.custom_setup_recipes.get_inner();
        let custom_shutdown_recipes_binding = args.custom_shutdown_recipes.get_inner();
        let custom_undeploy_recipes_binding = args.custom_undeploy_recipes.get_inner();
        let drain_elb_on_shutdown_binding = args.drain_elb_on_shutdown.get_inner();
        let ebs_volumes_binding = args.ebs_volumes.get_inner();
        let elastic_load_balancer_binding = args.elastic_load_balancer.get_inner();
        let install_updates_on_boot_binding = args.install_updates_on_boot.get_inner();
        let instance_shutdown_timeout_binding = args
            .instance_shutdown_timeout
            .get_inner();
        let load_based_auto_scaling_binding = args.load_based_auto_scaling.get_inner();
        let name_binding = args.name.get_inner();
        let nodejs_version_binding = args.nodejs_version.get_inner();
        let stack_id_binding = args.stack_id.get_inner();
        let system_packages_binding = args.system_packages.get_inner();
        let tags_binding = args.tags.get_inner();
        let use_ebs_optimized_instances_binding = args
            .use_ebs_optimized_instances
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opsworks/nodejsAppLayer:NodejsAppLayer".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoAssignElasticIps".into(),
                    value: &auto_assign_elastic_ips_binding,
                },
                register_interface::ObjectField {
                    name: "autoAssignPublicIps".into(),
                    value: &auto_assign_public_ips_binding,
                },
                register_interface::ObjectField {
                    name: "autoHealing".into(),
                    value: &auto_healing_binding,
                },
                register_interface::ObjectField {
                    name: "cloudwatchConfiguration".into(),
                    value: &cloudwatch_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "customConfigureRecipes".into(),
                    value: &custom_configure_recipes_binding,
                },
                register_interface::ObjectField {
                    name: "customDeployRecipes".into(),
                    value: &custom_deploy_recipes_binding,
                },
                register_interface::ObjectField {
                    name: "customInstanceProfileArn".into(),
                    value: &custom_instance_profile_arn_binding,
                },
                register_interface::ObjectField {
                    name: "customJson".into(),
                    value: &custom_json_binding,
                },
                register_interface::ObjectField {
                    name: "customSecurityGroupIds".into(),
                    value: &custom_security_group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "customSetupRecipes".into(),
                    value: &custom_setup_recipes_binding,
                },
                register_interface::ObjectField {
                    name: "customShutdownRecipes".into(),
                    value: &custom_shutdown_recipes_binding,
                },
                register_interface::ObjectField {
                    name: "customUndeployRecipes".into(),
                    value: &custom_undeploy_recipes_binding,
                },
                register_interface::ObjectField {
                    name: "drainElbOnShutdown".into(),
                    value: &drain_elb_on_shutdown_binding,
                },
                register_interface::ObjectField {
                    name: "ebsVolumes".into(),
                    value: &ebs_volumes_binding,
                },
                register_interface::ObjectField {
                    name: "elasticLoadBalancer".into(),
                    value: &elastic_load_balancer_binding,
                },
                register_interface::ObjectField {
                    name: "installUpdatesOnBoot".into(),
                    value: &install_updates_on_boot_binding,
                },
                register_interface::ObjectField {
                    name: "instanceShutdownTimeout".into(),
                    value: &instance_shutdown_timeout_binding,
                },
                register_interface::ObjectField {
                    name: "loadBasedAutoScaling".into(),
                    value: &load_based_auto_scaling_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nodejsVersion".into(),
                    value: &nodejs_version_binding,
                },
                register_interface::ObjectField {
                    name: "stackId".into(),
                    value: &stack_id_binding,
                },
                register_interface::ObjectField {
                    name: "systemPackages".into(),
                    value: &system_packages_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "useEbsOptimizedInstances".into(),
                    value: &use_ebs_optimized_instances_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "autoAssignElasticIps".into(),
                },
                register_interface::ResultField {
                    name: "autoAssignPublicIps".into(),
                },
                register_interface::ResultField {
                    name: "autoHealing".into(),
                },
                register_interface::ResultField {
                    name: "cloudwatchConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "customConfigureRecipes".into(),
                },
                register_interface::ResultField {
                    name: "customDeployRecipes".into(),
                },
                register_interface::ResultField {
                    name: "customInstanceProfileArn".into(),
                },
                register_interface::ResultField {
                    name: "customJson".into(),
                },
                register_interface::ResultField {
                    name: "customSecurityGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "customSetupRecipes".into(),
                },
                register_interface::ResultField {
                    name: "customShutdownRecipes".into(),
                },
                register_interface::ResultField {
                    name: "customUndeployRecipes".into(),
                },
                register_interface::ResultField {
                    name: "drainElbOnShutdown".into(),
                },
                register_interface::ResultField {
                    name: "ebsVolumes".into(),
                },
                register_interface::ResultField {
                    name: "elasticLoadBalancer".into(),
                },
                register_interface::ResultField {
                    name: "installUpdatesOnBoot".into(),
                },
                register_interface::ResultField {
                    name: "instanceShutdownTimeout".into(),
                },
                register_interface::ResultField {
                    name: "loadBasedAutoScaling".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nodejsVersion".into(),
                },
                register_interface::ResultField {
                    name: "stackId".into(),
                },
                register_interface::ResultField {
                    name: "systemPackages".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "useEbsOptimizedInstances".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NodejsAppLayerResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auto_assign_elastic_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoAssignElasticIps").unwrap(),
            ),
            auto_assign_public_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoAssignPublicIps").unwrap(),
            ),
            auto_healing: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoHealing").unwrap(),
            ),
            cloudwatch_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudwatchConfiguration").unwrap(),
            ),
            custom_configure_recipes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customConfigureRecipes").unwrap(),
            ),
            custom_deploy_recipes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customDeployRecipes").unwrap(),
            ),
            custom_instance_profile_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customInstanceProfileArn").unwrap(),
            ),
            custom_json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customJson").unwrap(),
            ),
            custom_security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customSecurityGroupIds").unwrap(),
            ),
            custom_setup_recipes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customSetupRecipes").unwrap(),
            ),
            custom_shutdown_recipes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customShutdownRecipes").unwrap(),
            ),
            custom_undeploy_recipes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customUndeployRecipes").unwrap(),
            ),
            drain_elb_on_shutdown: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("drainElbOnShutdown").unwrap(),
            ),
            ebs_volumes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsVolumes").unwrap(),
            ),
            elastic_load_balancer: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("elasticLoadBalancer").unwrap(),
            ),
            install_updates_on_boot: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("installUpdatesOnBoot").unwrap(),
            ),
            instance_shutdown_timeout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceShutdownTimeout").unwrap(),
            ),
            load_based_auto_scaling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBasedAutoScaling").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            nodejs_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodejsVersion").unwrap(),
            ),
            stack_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stackId").unwrap(),
            ),
            system_packages: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("systemPackages").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            use_ebs_optimized_instances: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("useEbsOptimizedInstances").unwrap(),
            ),
        }
    }
}
