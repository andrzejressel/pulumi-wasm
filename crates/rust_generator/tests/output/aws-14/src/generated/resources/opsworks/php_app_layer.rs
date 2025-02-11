/// Provides an OpsWorks PHP application layer resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let app = php_app_layer::create(
///         "app",
///         PhpAppLayerArgs::builder().stack_id("${main.id}").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import OpsWorks PHP Application Layers using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:opsworks/phpAppLayer:PhpAppLayer bar 00000000-0000-0000-0000-000000000000
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod php_app_layer {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PhpAppLayerArgs {
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
            Option<super::super::types::opsworks::PhpAppLayerCloudwatchConfiguration>,
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
            Option<Vec<super::super::types::opsworks::PhpAppLayerEbsVolume>>,
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
        #[builder(into, default)]
        pub load_based_auto_scaling: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::opsworks::PhpAppLayerLoadBasedAutoScaling>,
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
    pub struct PhpAppLayerResult {
        /// The Amazon Resource Name(ARN) of the layer.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Whether to automatically assign an elastic IP address to the layer's instances.
        pub auto_assign_elastic_ips: pulumi_gestalt_rust::Output<Option<bool>>,
        /// For stacks belonging to a VPC, whether to automatically assign a public IP address to each of the layer's instances.
        pub auto_assign_public_ips: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether to enable auto-healing for the layer.
        pub auto_healing: pulumi_gestalt_rust::Output<Option<bool>>,
        pub cloudwatch_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::opsworks::PhpAppLayerCloudwatchConfiguration>,
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
            Vec<super::super::types::opsworks::PhpAppLayerEbsVolume>,
        >,
        /// Name of an Elastic Load Balancer to attach to this layer
        pub elastic_load_balancer: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether to install OS and package updates on each instance when it boots.
        pub install_updates_on_boot: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The time, in seconds, that OpsWorks will wait for Chef to complete after triggering the Shutdown event.
        pub instance_shutdown_timeout: pulumi_gestalt_rust::Output<Option<i32>>,
        pub load_based_auto_scaling: pulumi_gestalt_rust::Output<
            super::super::types::opsworks::PhpAppLayerLoadBasedAutoScaling,
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PhpAppLayerArgs,
    ) -> PhpAppLayerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_assign_elastic_ips_binding = args
            .auto_assign_elastic_ips
            .get_output(context);
        let auto_assign_public_ips_binding = args
            .auto_assign_public_ips
            .get_output(context);
        let auto_healing_binding = args.auto_healing.get_output(context);
        let cloudwatch_configuration_binding = args
            .cloudwatch_configuration
            .get_output(context);
        let custom_configure_recipes_binding = args
            .custom_configure_recipes
            .get_output(context);
        let custom_deploy_recipes_binding = args
            .custom_deploy_recipes
            .get_output(context);
        let custom_instance_profile_arn_binding = args
            .custom_instance_profile_arn
            .get_output(context);
        let custom_json_binding = args.custom_json.get_output(context);
        let custom_security_group_ids_binding = args
            .custom_security_group_ids
            .get_output(context);
        let custom_setup_recipes_binding = args.custom_setup_recipes.get_output(context);
        let custom_shutdown_recipes_binding = args
            .custom_shutdown_recipes
            .get_output(context);
        let custom_undeploy_recipes_binding = args
            .custom_undeploy_recipes
            .get_output(context);
        let drain_elb_on_shutdown_binding = args
            .drain_elb_on_shutdown
            .get_output(context);
        let ebs_volumes_binding = args.ebs_volumes.get_output(context);
        let elastic_load_balancer_binding = args
            .elastic_load_balancer
            .get_output(context);
        let install_updates_on_boot_binding = args
            .install_updates_on_boot
            .get_output(context);
        let instance_shutdown_timeout_binding = args
            .instance_shutdown_timeout
            .get_output(context);
        let load_based_auto_scaling_binding = args
            .load_based_auto_scaling
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let stack_id_binding = args.stack_id.get_output(context);
        let system_packages_binding = args.system_packages.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let use_ebs_optimized_instances_binding = args
            .use_ebs_optimized_instances
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:opsworks/phpAppLayer:PhpAppLayer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoAssignElasticIps".into(),
                    value: &auto_assign_elastic_ips_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoAssignPublicIps".into(),
                    value: &auto_assign_public_ips_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoHealing".into(),
                    value: &auto_healing_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudwatchConfiguration".into(),
                    value: &cloudwatch_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customConfigureRecipes".into(),
                    value: &custom_configure_recipes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customDeployRecipes".into(),
                    value: &custom_deploy_recipes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customInstanceProfileArn".into(),
                    value: &custom_instance_profile_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customJson".into(),
                    value: &custom_json_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customSecurityGroupIds".into(),
                    value: &custom_security_group_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customSetupRecipes".into(),
                    value: &custom_setup_recipes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customShutdownRecipes".into(),
                    value: &custom_shutdown_recipes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customUndeployRecipes".into(),
                    value: &custom_undeploy_recipes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "drainElbOnShutdown".into(),
                    value: &drain_elb_on_shutdown_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ebsVolumes".into(),
                    value: &ebs_volumes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "elasticLoadBalancer".into(),
                    value: &elastic_load_balancer_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "installUpdatesOnBoot".into(),
                    value: &install_updates_on_boot_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceShutdownTimeout".into(),
                    value: &instance_shutdown_timeout_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadBasedAutoScaling".into(),
                    value: &load_based_auto_scaling_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stackId".into(),
                    value: &stack_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "systemPackages".into(),
                    value: &system_packages_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "useEbsOptimizedInstances".into(),
                    value: &use_ebs_optimized_instances_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PhpAppLayerResult {
            arn: o.get_field("arn"),
            auto_assign_elastic_ips: o.get_field("autoAssignElasticIps"),
            auto_assign_public_ips: o.get_field("autoAssignPublicIps"),
            auto_healing: o.get_field("autoHealing"),
            cloudwatch_configuration: o.get_field("cloudwatchConfiguration"),
            custom_configure_recipes: o.get_field("customConfigureRecipes"),
            custom_deploy_recipes: o.get_field("customDeployRecipes"),
            custom_instance_profile_arn: o.get_field("customInstanceProfileArn"),
            custom_json: o.get_field("customJson"),
            custom_security_group_ids: o.get_field("customSecurityGroupIds"),
            custom_setup_recipes: o.get_field("customSetupRecipes"),
            custom_shutdown_recipes: o.get_field("customShutdownRecipes"),
            custom_undeploy_recipes: o.get_field("customUndeployRecipes"),
            drain_elb_on_shutdown: o.get_field("drainElbOnShutdown"),
            ebs_volumes: o.get_field("ebsVolumes"),
            elastic_load_balancer: o.get_field("elasticLoadBalancer"),
            install_updates_on_boot: o.get_field("installUpdatesOnBoot"),
            instance_shutdown_timeout: o.get_field("instanceShutdownTimeout"),
            load_based_auto_scaling: o.get_field("loadBasedAutoScaling"),
            name: o.get_field("name"),
            stack_id: o.get_field("stackId"),
            system_packages: o.get_field("systemPackages"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            use_ebs_optimized_instances: o.get_field("useEbsOptimizedInstances"),
        }
    }
}
