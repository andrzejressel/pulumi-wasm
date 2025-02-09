/// Provides an OpsWorks Java application layer resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let app = java_app_layer::create(
///         "app",
///         JavaAppLayerArgs::builder().stack_id("${main.id}").build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod java_app_layer {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct JavaAppLayerArgs {
        /// Keyword for the application container to use. Defaults to "tomcat".
        #[builder(into, default)]
        pub app_server: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Version of the selected application container to use. Defaults to "7".
        #[builder(into, default)]
        pub app_server_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to automatically assign an elastic IP address to the layer's instances.
        #[builder(into, default)]
        pub auto_assign_elastic_ips: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// For stacks belonging to a VPC, whether to automatically assign a public IP address to each of the layer's instances.
        #[builder(into, default)]
        pub auto_assign_public_ips: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether to enable auto-healing for the layer.
        #[builder(into, default)]
        pub auto_healing: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub cloudwatch_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::opsworks::JavaAppLayerCloudwatchConfiguration>,
        >,
        #[builder(into, default)]
        pub custom_configure_recipes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        #[builder(into, default)]
        pub custom_deploy_recipes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The ARN of an IAM profile that will be used for the layer's instances.
        #[builder(into, default)]
        pub custom_instance_profile_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Custom JSON attributes to apply to the layer.
        #[builder(into, default)]
        pub custom_json: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Ids for a set of security groups to apply to the layer's instances.
        #[builder(into, default)]
        pub custom_security_group_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        #[builder(into, default)]
        pub custom_setup_recipes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        #[builder(into, default)]
        pub custom_shutdown_recipes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        #[builder(into, default)]
        pub custom_undeploy_recipes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Whether to enable Elastic Load Balancing connection draining.
        #[builder(into, default)]
        pub drain_elb_on_shutdown: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// `ebs_volume` blocks, as described below, will each create an EBS volume and connect it to the layer's instances.
        #[builder(into, default)]
        pub ebs_volumes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::opsworks::JavaAppLayerEbsVolume>>,
        >,
        /// Name of an Elastic Load Balancer to attach to this layer
        #[builder(into, default)]
        pub elastic_load_balancer: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to install OS and package updates on each instance when it boots.
        #[builder(into, default)]
        pub install_updates_on_boot: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The time, in seconds, that OpsWorks will wait for Chef to complete after triggering the Shutdown event.
        #[builder(into, default)]
        pub instance_shutdown_timeout: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Options to set for the JVM.
        #[builder(into, default)]
        pub jvm_options: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Keyword for the type of JVM to use. Defaults to `openjdk`.
        #[builder(into, default)]
        pub jvm_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Version of JVM to use. Defaults to "7".
        #[builder(into, default)]
        pub jvm_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub load_based_auto_scaling: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::opsworks::JavaAppLayerLoadBasedAutoScaling>,
        >,
        /// A human-readable name for the layer.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the stack the layer will belong to.
        #[builder(into)]
        pub stack_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Names of a set of system packages to install on the layer's instances.
        #[builder(into, default)]
        pub system_packages: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// The following extra optional arguments, all lists of Chef recipe names, allow
        /// custom Chef recipes to be applied to layer instances at the five different
        /// lifecycle events, if custom cookbooks are enabled on the layer's stack:
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whether to use EBS-optimized instances.
        #[builder(into, default)]
        pub use_ebs_optimized_instances: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct JavaAppLayerResult {
        /// Keyword for the application container to use. Defaults to "tomcat".
        pub app_server: pulumi_gestalt_rust::Output<Option<String>>,
        /// Version of the selected application container to use. Defaults to "7".
        pub app_server_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Amazon Resource Name(ARN) of the layer.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Whether to automatically assign an elastic IP address to the layer's instances.
        pub auto_assign_elastic_ips: pulumi_gestalt_rust::Output<Option<bool>>,
        /// For stacks belonging to a VPC, whether to automatically assign a public IP address to each of the layer's instances.
        pub auto_assign_public_ips: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether to enable auto-healing for the layer.
        pub auto_healing: pulumi_gestalt_rust::Output<Option<bool>>,
        pub cloudwatch_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::opsworks::JavaAppLayerCloudwatchConfiguration>,
        >,
        pub custom_configure_recipes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub custom_deploy_recipes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The ARN of an IAM profile that will be used for the layer's instances.
        pub custom_instance_profile_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Custom JSON attributes to apply to the layer.
        pub custom_json: pulumi_gestalt_rust::Output<Option<String>>,
        /// Ids for a set of security groups to apply to the layer's instances.
        pub custom_security_group_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub custom_setup_recipes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub custom_shutdown_recipes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub custom_undeploy_recipes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Whether to enable Elastic Load Balancing connection draining.
        pub drain_elb_on_shutdown: pulumi_gestalt_rust::Output<Option<bool>>,
        /// `ebs_volume` blocks, as described below, will each create an EBS volume and connect it to the layer's instances.
        pub ebs_volumes: pulumi_gestalt_rust::Output<
            Vec<super::super::types::opsworks::JavaAppLayerEbsVolume>,
        >,
        /// Name of an Elastic Load Balancer to attach to this layer
        pub elastic_load_balancer: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether to install OS and package updates on each instance when it boots.
        pub install_updates_on_boot: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The time, in seconds, that OpsWorks will wait for Chef to complete after triggering the Shutdown event.
        pub instance_shutdown_timeout: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Options to set for the JVM.
        pub jvm_options: pulumi_gestalt_rust::Output<Option<String>>,
        /// Keyword for the type of JVM to use. Defaults to `openjdk`.
        pub jvm_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Version of JVM to use. Defaults to "7".
        pub jvm_version: pulumi_gestalt_rust::Output<Option<String>>,
        pub load_based_auto_scaling: pulumi_gestalt_rust::Output<
            super::super::types::opsworks::JavaAppLayerLoadBasedAutoScaling,
        >,
        /// A human-readable name for the layer.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// ID of the stack the layer will belong to.
        pub stack_id: pulumi_gestalt_rust::Output<String>,
        /// Names of a set of system packages to install on the layer's instances.
        pub system_packages: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// The following extra optional arguments, all lists of Chef recipe names, allow
        /// custom Chef recipes to be applied to layer instances at the five different
        /// lifecycle events, if custom cookbooks are enabled on the layer's stack:
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whether to use EBS-optimized instances.
        pub use_ebs_optimized_instances: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: JavaAppLayerArgs,
    ) -> JavaAppLayerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let app_server_binding_1 = args.app_server.get_output(context);
        let app_server_binding = app_server_binding_1.get_inner();
        let app_server_version_binding_1 = args.app_server_version.get_output(context);
        let app_server_version_binding = app_server_version_binding_1.get_inner();
        let auto_assign_elastic_ips_binding_1 = args
            .auto_assign_elastic_ips
            .get_output(context);
        let auto_assign_elastic_ips_binding = auto_assign_elastic_ips_binding_1
            .get_inner();
        let auto_assign_public_ips_binding_1 = args
            .auto_assign_public_ips
            .get_output(context);
        let auto_assign_public_ips_binding = auto_assign_public_ips_binding_1
            .get_inner();
        let auto_healing_binding_1 = args.auto_healing.get_output(context);
        let auto_healing_binding = auto_healing_binding_1.get_inner();
        let cloudwatch_configuration_binding_1 = args
            .cloudwatch_configuration
            .get_output(context);
        let cloudwatch_configuration_binding = cloudwatch_configuration_binding_1
            .get_inner();
        let custom_configure_recipes_binding_1 = args
            .custom_configure_recipes
            .get_output(context);
        let custom_configure_recipes_binding = custom_configure_recipes_binding_1
            .get_inner();
        let custom_deploy_recipes_binding_1 = args
            .custom_deploy_recipes
            .get_output(context);
        let custom_deploy_recipes_binding = custom_deploy_recipes_binding_1.get_inner();
        let custom_instance_profile_arn_binding_1 = args
            .custom_instance_profile_arn
            .get_output(context);
        let custom_instance_profile_arn_binding = custom_instance_profile_arn_binding_1
            .get_inner();
        let custom_json_binding_1 = args.custom_json.get_output(context);
        let custom_json_binding = custom_json_binding_1.get_inner();
        let custom_security_group_ids_binding_1 = args
            .custom_security_group_ids
            .get_output(context);
        let custom_security_group_ids_binding = custom_security_group_ids_binding_1
            .get_inner();
        let custom_setup_recipes_binding_1 = args
            .custom_setup_recipes
            .get_output(context);
        let custom_setup_recipes_binding = custom_setup_recipes_binding_1.get_inner();
        let custom_shutdown_recipes_binding_1 = args
            .custom_shutdown_recipes
            .get_output(context);
        let custom_shutdown_recipes_binding = custom_shutdown_recipes_binding_1
            .get_inner();
        let custom_undeploy_recipes_binding_1 = args
            .custom_undeploy_recipes
            .get_output(context);
        let custom_undeploy_recipes_binding = custom_undeploy_recipes_binding_1
            .get_inner();
        let drain_elb_on_shutdown_binding_1 = args
            .drain_elb_on_shutdown
            .get_output(context);
        let drain_elb_on_shutdown_binding = drain_elb_on_shutdown_binding_1.get_inner();
        let ebs_volumes_binding_1 = args.ebs_volumes.get_output(context);
        let ebs_volumes_binding = ebs_volumes_binding_1.get_inner();
        let elastic_load_balancer_binding_1 = args
            .elastic_load_balancer
            .get_output(context);
        let elastic_load_balancer_binding = elastic_load_balancer_binding_1.get_inner();
        let install_updates_on_boot_binding_1 = args
            .install_updates_on_boot
            .get_output(context);
        let install_updates_on_boot_binding = install_updates_on_boot_binding_1
            .get_inner();
        let instance_shutdown_timeout_binding_1 = args
            .instance_shutdown_timeout
            .get_output(context);
        let instance_shutdown_timeout_binding = instance_shutdown_timeout_binding_1
            .get_inner();
        let jvm_options_binding_1 = args.jvm_options.get_output(context);
        let jvm_options_binding = jvm_options_binding_1.get_inner();
        let jvm_type_binding_1 = args.jvm_type.get_output(context);
        let jvm_type_binding = jvm_type_binding_1.get_inner();
        let jvm_version_binding_1 = args.jvm_version.get_output(context);
        let jvm_version_binding = jvm_version_binding_1.get_inner();
        let load_based_auto_scaling_binding_1 = args
            .load_based_auto_scaling
            .get_output(context);
        let load_based_auto_scaling_binding = load_based_auto_scaling_binding_1
            .get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let stack_id_binding_1 = args.stack_id.get_output(context);
        let stack_id_binding = stack_id_binding_1.get_inner();
        let system_packages_binding_1 = args.system_packages.get_output(context);
        let system_packages_binding = system_packages_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let use_ebs_optimized_instances_binding_1 = args
            .use_ebs_optimized_instances
            .get_output(context);
        let use_ebs_optimized_instances_binding = use_ebs_optimized_instances_binding_1
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opsworks/javaAppLayer:JavaAppLayer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appServer".into(),
                    value: &app_server_binding,
                },
                register_interface::ObjectField {
                    name: "appServerVersion".into(),
                    value: &app_server_version_binding,
                },
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
                    name: "jvmOptions".into(),
                    value: &jvm_options_binding,
                },
                register_interface::ObjectField {
                    name: "jvmType".into(),
                    value: &jvm_type_binding,
                },
                register_interface::ObjectField {
                    name: "jvmVersion".into(),
                    value: &jvm_version_binding,
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
        JavaAppLayerResult {
            app_server: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appServer"),
            ),
            app_server_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appServerVersion"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            auto_assign_elastic_ips: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoAssignElasticIps"),
            ),
            auto_assign_public_ips: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoAssignPublicIps"),
            ),
            auto_healing: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoHealing"),
            ),
            cloudwatch_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cloudwatchConfiguration"),
            ),
            custom_configure_recipes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customConfigureRecipes"),
            ),
            custom_deploy_recipes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customDeployRecipes"),
            ),
            custom_instance_profile_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customInstanceProfileArn"),
            ),
            custom_json: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customJson"),
            ),
            custom_security_group_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customSecurityGroupIds"),
            ),
            custom_setup_recipes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customSetupRecipes"),
            ),
            custom_shutdown_recipes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customShutdownRecipes"),
            ),
            custom_undeploy_recipes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customUndeployRecipes"),
            ),
            drain_elb_on_shutdown: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("drainElbOnShutdown"),
            ),
            ebs_volumes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ebsVolumes"),
            ),
            elastic_load_balancer: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("elasticLoadBalancer"),
            ),
            install_updates_on_boot: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("installUpdatesOnBoot"),
            ),
            instance_shutdown_timeout: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceShutdownTimeout"),
            ),
            jvm_options: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("jvmOptions"),
            ),
            jvm_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("jvmType"),
            ),
            jvm_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("jvmVersion"),
            ),
            load_based_auto_scaling: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loadBasedAutoScaling"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            stack_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("stackId"),
            ),
            system_packages: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("systemPackages"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            use_ebs_optimized_instances: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("useEbsOptimizedInstances"),
            ),
        }
    }
}
