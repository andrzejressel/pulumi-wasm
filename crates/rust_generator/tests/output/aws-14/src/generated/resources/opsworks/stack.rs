/// Provides an OpsWorks stack resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   main:
///     type: aws:opsworks:Stack
///     properties:
///       name: awesome-stack
///       region: us-west-1
///       serviceRoleArn: ${opsworksAwsIamRole.arn}
///       defaultInstanceProfileArn: ${opsworks.arn}
///       tags:
///         Name: foobar-stack
///       customJson: |
///         {
///          "foobar": {
///             "version": "1.0.0"
///           }
///         }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import OpsWorks stacks using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:opsworks/stack:Stack bar 00000000-0000-0000-0000-000000000000
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod stack {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StackArgs {
        /// If set to `"LATEST"`, OpsWorks will automatically install the latest version.
        #[builder(into, default)]
        pub agent_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If `manage_berkshelf` is enabled, the version of Berkshelf to use.
        #[builder(into, default)]
        pub berkshelf_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Color to paint next to the stack's resources in the OpsWorks console.
        #[builder(into, default)]
        pub color: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the configuration manager to use. Defaults to "Chef".
        #[builder(into, default)]
        pub configuration_manager_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Version of the configuration manager to use. Defaults to "11.4".
        #[builder(into, default)]
        pub configuration_manager_version: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// When `use_custom_cookbooks` is set, provide this sub-object as described below.
        #[builder(into, default)]
        pub custom_cookbooks_sources: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::opsworks::StackCustomCookbooksSource>>,
        >,
        /// Custom JSON attributes to apply to the entire stack.
        #[builder(into, default)]
        pub custom_json: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the availability zone where instances will be created by default.
        /// Cannot be set when `vpc_id` is set.
        #[builder(into, default)]
        pub default_availability_zone: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ARN of an IAM Instance Profile that created instances will have by default.
        #[builder(into)]
        pub default_instance_profile_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of OS that will be installed on instances by default.
        #[builder(into, default)]
        pub default_os: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the type of root device instances will have by default.
        #[builder(into, default)]
        pub default_root_device_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the SSH keypair that instances will have by default.
        #[builder(into, default)]
        pub default_ssh_key_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the subnet in which instances will be created by default.
        /// Required if `vpc_id` is set to a VPC other than the default VPC, and forbidden if it isn't.
        #[builder(into, default)]
        pub default_subnet_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Keyword representing the naming scheme that will be used for instance hostnames within this stack.
        #[builder(into, default)]
        pub hostname_theme: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Boolean value controlling whether Opsworks will run Berkshelf for this stack.
        #[builder(into, default)]
        pub manage_berkshelf: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the stack.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the region where the stack will exist.
        #[builder(into)]
        pub region: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ARN of an IAM role that the OpsWorks service will act as.
        #[builder(into)]
        pub service_role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource.
        /// If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Boolean value controlling whether the custom cookbook settings are enabled.
        #[builder(into, default)]
        pub use_custom_cookbooks: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Boolean value controlling whether the standard OpsWorks security groups apply to created instances.
        #[builder(into, default)]
        pub use_opsworks_security_groups: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// ID of the VPC that this stack belongs to.
        /// Defaults to the region's default VPC.
        #[builder(into, default)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct StackResult {
        /// If set to `"LATEST"`, OpsWorks will automatically install the latest version.
        pub agent_version: pulumi_gestalt_rust::Output<String>,
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// If `manage_berkshelf` is enabled, the version of Berkshelf to use.
        pub berkshelf_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// Color to paint next to the stack's resources in the OpsWorks console.
        pub color: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the configuration manager to use. Defaults to "Chef".
        pub configuration_manager_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Version of the configuration manager to use. Defaults to "11.4".
        pub configuration_manager_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// When `use_custom_cookbooks` is set, provide this sub-object as described below.
        pub custom_cookbooks_sources: pulumi_gestalt_rust::Output<
            Vec<super::super::types::opsworks::StackCustomCookbooksSource>,
        >,
        /// Custom JSON attributes to apply to the entire stack.
        pub custom_json: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the availability zone where instances will be created by default.
        /// Cannot be set when `vpc_id` is set.
        pub default_availability_zone: pulumi_gestalt_rust::Output<String>,
        /// The ARN of an IAM Instance Profile that created instances will have by default.
        pub default_instance_profile_arn: pulumi_gestalt_rust::Output<String>,
        /// Name of OS that will be installed on instances by default.
        pub default_os: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the type of root device instances will have by default.
        pub default_root_device_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the SSH keypair that instances will have by default.
        pub default_ssh_key_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// ID of the subnet in which instances will be created by default.
        /// Required if `vpc_id` is set to a VPC other than the default VPC, and forbidden if it isn't.
        pub default_subnet_id: pulumi_gestalt_rust::Output<String>,
        /// Keyword representing the naming scheme that will be used for instance hostnames within this stack.
        pub hostname_theme: pulumi_gestalt_rust::Output<Option<String>>,
        /// Boolean value controlling whether Opsworks will run Berkshelf for this stack.
        pub manage_berkshelf: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the stack.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the region where the stack will exist.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The ARN of an IAM role that the OpsWorks service will act as.
        pub service_role_arn: pulumi_gestalt_rust::Output<String>,
        pub stack_endpoint: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource.
        /// If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Boolean value controlling whether the custom cookbook settings are enabled.
        pub use_custom_cookbooks: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Boolean value controlling whether the standard OpsWorks security groups apply to created instances.
        pub use_opsworks_security_groups: pulumi_gestalt_rust::Output<Option<bool>>,
        /// ID of the VPC that this stack belongs to.
        /// Defaults to the region's default VPC.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: StackArgs,
    ) -> StackResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let agent_version_binding = args.agent_version.get_output(context);
        let berkshelf_version_binding = args.berkshelf_version.get_output(context);
        let color_binding = args.color.get_output(context);
        let configuration_manager_name_binding = args
            .configuration_manager_name
            .get_output(context);
        let configuration_manager_version_binding = args
            .configuration_manager_version
            .get_output(context);
        let custom_cookbooks_sources_binding = args
            .custom_cookbooks_sources
            .get_output(context);
        let custom_json_binding = args.custom_json.get_output(context);
        let default_availability_zone_binding = args
            .default_availability_zone
            .get_output(context);
        let default_instance_profile_arn_binding = args
            .default_instance_profile_arn
            .get_output(context);
        let default_os_binding = args.default_os.get_output(context);
        let default_root_device_type_binding = args
            .default_root_device_type
            .get_output(context);
        let default_ssh_key_name_binding = args.default_ssh_key_name.get_output(context);
        let default_subnet_id_binding = args.default_subnet_id.get_output(context);
        let hostname_theme_binding = args.hostname_theme.get_output(context);
        let manage_berkshelf_binding = args.manage_berkshelf.get_output(context);
        let name_binding = args.name.get_output(context);
        let region_binding = args.region.get_output(context);
        let service_role_arn_binding = args.service_role_arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let use_custom_cookbooks_binding = args.use_custom_cookbooks.get_output(context);
        let use_opsworks_security_groups_binding = args
            .use_opsworks_security_groups
            .get_output(context);
        let vpc_id_binding = args.vpc_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:opsworks/stack:Stack".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "agentVersion".into(),
                    value: agent_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "berkshelfVersion".into(),
                    value: berkshelf_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "color".into(),
                    value: color_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configurationManagerName".into(),
                    value: configuration_manager_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configurationManagerVersion".into(),
                    value: configuration_manager_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customCookbooksSources".into(),
                    value: custom_cookbooks_sources_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customJson".into(),
                    value: custom_json_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultAvailabilityZone".into(),
                    value: default_availability_zone_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultInstanceProfileArn".into(),
                    value: default_instance_profile_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultOs".into(),
                    value: default_os_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultRootDeviceType".into(),
                    value: default_root_device_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultSshKeyName".into(),
                    value: default_ssh_key_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultSubnetId".into(),
                    value: default_subnet_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostnameTheme".into(),
                    value: hostname_theme_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "manageBerkshelf".into(),
                    value: manage_berkshelf_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
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
                    name: "useCustomCookbooks".into(),
                    value: use_custom_cookbooks_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "useOpsworksSecurityGroups".into(),
                    value: use_opsworks_security_groups_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcId".into(),
                    value: vpc_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        StackResult {
            agent_version: o.get_field("agentVersion"),
            arn: o.get_field("arn"),
            berkshelf_version: o.get_field("berkshelfVersion"),
            color: o.get_field("color"),
            configuration_manager_name: o.get_field("configurationManagerName"),
            configuration_manager_version: o.get_field("configurationManagerVersion"),
            custom_cookbooks_sources: o.get_field("customCookbooksSources"),
            custom_json: o.get_field("customJson"),
            default_availability_zone: o.get_field("defaultAvailabilityZone"),
            default_instance_profile_arn: o.get_field("defaultInstanceProfileArn"),
            default_os: o.get_field("defaultOs"),
            default_root_device_type: o.get_field("defaultRootDeviceType"),
            default_ssh_key_name: o.get_field("defaultSshKeyName"),
            default_subnet_id: o.get_field("defaultSubnetId"),
            hostname_theme: o.get_field("hostnameTheme"),
            manage_berkshelf: o.get_field("manageBerkshelf"),
            name: o.get_field("name"),
            region: o.get_field("region"),
            service_role_arn: o.get_field("serviceRoleArn"),
            stack_endpoint: o.get_field("stackEndpoint"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            use_custom_cookbooks: o.get_field("useCustomCookbooks"),
            use_opsworks_security_groups: o.get_field("useOpsworksSecurityGroups"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
