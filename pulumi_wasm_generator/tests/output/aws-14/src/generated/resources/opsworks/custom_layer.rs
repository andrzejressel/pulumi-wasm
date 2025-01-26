/// Provides an OpsWorks custom layer resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let custlayer = custom_layer::create(
///         "custlayer",
///         CustomLayerArgs::builder()
///             .name("My Awesome Custom Layer")
///             .short_name("awesome")
///             .stack_id("${main.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import OpsWorks Custom Layers using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:opsworks/customLayer:CustomLayer bar 00000000-0000-0000-0000-000000000000
/// ```
pub mod custom_layer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomLayerArgs {
        /// Whether to automatically assign an elastic IP address to the layer's instances.
        #[builder(into, default)]
        pub auto_assign_elastic_ips: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// For stacks belonging to a VPC, whether to automatically assign a public IP address to each of the layer's instances.
        #[builder(into, default)]
        pub auto_assign_public_ips: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Whether to enable auto-healing for the layer.
        #[builder(into, default)]
        pub auto_healing: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Will create an EBS volume and connect it to the layer's instances. See Cloudwatch Configuration.
        #[builder(into, default)]
        pub cloudwatch_configuration: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::opsworks::CustomLayerCloudwatchConfiguration>,
        >,
        #[builder(into, default)]
        pub custom_configure_recipes: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        #[builder(into, default)]
        pub custom_deploy_recipes: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ARN of an IAM profile that will be used for the layer's instances.
        #[builder(into, default)]
        pub custom_instance_profile_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Custom JSON attributes to apply to the layer.
        #[builder(into, default)]
        pub custom_json: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Ids for a set of security groups to apply to the layer's instances.
        #[builder(into, default)]
        pub custom_security_group_ids: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        #[builder(into, default)]
        pub custom_setup_recipes: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        #[builder(into, default)]
        pub custom_shutdown_recipes: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        #[builder(into, default)]
        pub custom_undeploy_recipes: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Whether to enable Elastic Load Balancing connection draining.
        #[builder(into, default)]
        pub drain_elb_on_shutdown: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Will create an EBS volume and connect it to the layer's instances. See EBS Volume.
        #[builder(into, default)]
        pub ebs_volumes: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::opsworks::CustomLayerEbsVolume>>,
        >,
        /// Name of an Elastic Load Balancer to attach to this layer
        #[builder(into, default)]
        pub elastic_load_balancer: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether to install OS and package updates on each instance when it boots.
        #[builder(into, default)]
        pub install_updates_on_boot: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The time, in seconds, that OpsWorks will wait for Chef to complete after triggering the Shutdown event.
        #[builder(into, default)]
        pub instance_shutdown_timeout: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Load-based auto scaling configuration. See Load Based AutoScaling
        #[builder(into, default)]
        pub load_based_auto_scaling: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::opsworks::CustomLayerLoadBasedAutoScaling>,
        >,
        /// A human-readable name for the layer.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A short, machine-readable name for the layer, which will be used to identify it in the Chef node JSON.
        #[builder(into)]
        pub short_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// ID of the stack the layer will belong to.
        #[builder(into)]
        pub stack_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Names of a set of system packages to install on the layer's instances.
        #[builder(into, default)]
        pub system_packages: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// The following extra optional arguments, all lists of Chef recipe names, allow
        /// custom Chef recipes to be applied to layer instances at the five different
        /// lifecycle events, if custom cookbooks are enabled on the layer's stack:
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whether to use EBS-optimized instances.
        #[builder(into, default)]
        pub use_ebs_optimized_instances: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct CustomLayerResult {
        /// The Amazon Resource Name(ARN) of the layer.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Whether to automatically assign an elastic IP address to the layer's instances.
        pub auto_assign_elastic_ips: pulumi_wasm_rust::Output<Option<bool>>,
        /// For stacks belonging to a VPC, whether to automatically assign a public IP address to each of the layer's instances.
        pub auto_assign_public_ips: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to enable auto-healing for the layer.
        pub auto_healing: pulumi_wasm_rust::Output<Option<bool>>,
        /// Will create an EBS volume and connect it to the layer's instances. See Cloudwatch Configuration.
        pub cloudwatch_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::opsworks::CustomLayerCloudwatchConfiguration>,
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
        /// Will create an EBS volume and connect it to the layer's instances. See EBS Volume.
        pub ebs_volumes: pulumi_wasm_rust::Output<
            Vec<super::super::types::opsworks::CustomLayerEbsVolume>,
        >,
        /// Name of an Elastic Load Balancer to attach to this layer
        pub elastic_load_balancer: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to install OS and package updates on each instance when it boots.
        pub install_updates_on_boot: pulumi_wasm_rust::Output<Option<bool>>,
        /// The time, in seconds, that OpsWorks will wait for Chef to complete after triggering the Shutdown event.
        pub instance_shutdown_timeout: pulumi_wasm_rust::Output<Option<i32>>,
        /// Load-based auto scaling configuration. See Load Based AutoScaling
        pub load_based_auto_scaling: pulumi_wasm_rust::Output<
            super::super::types::opsworks::CustomLayerLoadBasedAutoScaling,
        >,
        /// A human-readable name for the layer.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A short, machine-readable name for the layer, which will be used to identify it in the Chef node JSON.
        pub short_name: pulumi_wasm_rust::Output<String>,
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CustomLayerArgs,
    ) -> CustomLayerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_assign_elastic_ips_binding = args
            .auto_assign_elastic_ips
            .get_output(context)
            .get_inner();
        let auto_assign_public_ips_binding = args
            .auto_assign_public_ips
            .get_output(context)
            .get_inner();
        let auto_healing_binding = args.auto_healing.get_output(context).get_inner();
        let cloudwatch_configuration_binding = args
            .cloudwatch_configuration
            .get_output(context)
            .get_inner();
        let custom_configure_recipes_binding = args
            .custom_configure_recipes
            .get_output(context)
            .get_inner();
        let custom_deploy_recipes_binding = args
            .custom_deploy_recipes
            .get_output(context)
            .get_inner();
        let custom_instance_profile_arn_binding = args
            .custom_instance_profile_arn
            .get_output(context)
            .get_inner();
        let custom_json_binding = args.custom_json.get_output(context).get_inner();
        let custom_security_group_ids_binding = args
            .custom_security_group_ids
            .get_output(context)
            .get_inner();
        let custom_setup_recipes_binding = args
            .custom_setup_recipes
            .get_output(context)
            .get_inner();
        let custom_shutdown_recipes_binding = args
            .custom_shutdown_recipes
            .get_output(context)
            .get_inner();
        let custom_undeploy_recipes_binding = args
            .custom_undeploy_recipes
            .get_output(context)
            .get_inner();
        let drain_elb_on_shutdown_binding = args
            .drain_elb_on_shutdown
            .get_output(context)
            .get_inner();
        let ebs_volumes_binding = args.ebs_volumes.get_output(context).get_inner();
        let elastic_load_balancer_binding = args
            .elastic_load_balancer
            .get_output(context)
            .get_inner();
        let install_updates_on_boot_binding = args
            .install_updates_on_boot
            .get_output(context)
            .get_inner();
        let instance_shutdown_timeout_binding = args
            .instance_shutdown_timeout
            .get_output(context)
            .get_inner();
        let load_based_auto_scaling_binding = args
            .load_based_auto_scaling
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let short_name_binding = args.short_name.get_output(context).get_inner();
        let stack_id_binding = args.stack_id.get_output(context).get_inner();
        let system_packages_binding = args
            .system_packages
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let use_ebs_optimized_instances_binding = args
            .use_ebs_optimized_instances
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opsworks/customLayer:CustomLayer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "shortName".into(),
                    value: &short_name_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        CustomLayerResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            auto_assign_elastic_ips: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoAssignElasticIps"),
            ),
            auto_assign_public_ips: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoAssignPublicIps"),
            ),
            auto_healing: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoHealing"),
            ),
            cloudwatch_configuration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cloudwatchConfiguration"),
            ),
            custom_configure_recipes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customConfigureRecipes"),
            ),
            custom_deploy_recipes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customDeployRecipes"),
            ),
            custom_instance_profile_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customInstanceProfileArn"),
            ),
            custom_json: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customJson"),
            ),
            custom_security_group_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customSecurityGroupIds"),
            ),
            custom_setup_recipes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customSetupRecipes"),
            ),
            custom_shutdown_recipes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customShutdownRecipes"),
            ),
            custom_undeploy_recipes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customUndeployRecipes"),
            ),
            drain_elb_on_shutdown: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("drainElbOnShutdown"),
            ),
            ebs_volumes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ebsVolumes"),
            ),
            elastic_load_balancer: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("elasticLoadBalancer"),
            ),
            install_updates_on_boot: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("installUpdatesOnBoot"),
            ),
            instance_shutdown_timeout: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceShutdownTimeout"),
            ),
            load_based_auto_scaling: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("loadBasedAutoScaling"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            short_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("shortName"),
            ),
            stack_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("stackId"),
            ),
            system_packages: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("systemPackages"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            use_ebs_optimized_instances: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("useEbsOptimizedInstances"),
            ),
        }
    }
}
