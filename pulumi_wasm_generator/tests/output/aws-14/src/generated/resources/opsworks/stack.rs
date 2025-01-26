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
pub mod stack {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StackArgs {
        /// If set to `"LATEST"`, OpsWorks will automatically install the latest version.
        #[builder(into, default)]
        pub agent_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// If `manage_berkshelf` is enabled, the version of Berkshelf to use.
        #[builder(into, default)]
        pub berkshelf_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Color to paint next to the stack's resources in the OpsWorks console.
        #[builder(into, default)]
        pub color: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the configuration manager to use. Defaults to "Chef".
        #[builder(into, default)]
        pub configuration_manager_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Version of the configuration manager to use. Defaults to "11.4".
        #[builder(into, default)]
        pub configuration_manager_version: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// When `use_custom_cookbooks` is set, provide this sub-object as described below.
        #[builder(into, default)]
        pub custom_cookbooks_sources: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::opsworks::StackCustomCookbooksSource>>,
        >,
        /// Custom JSON attributes to apply to the entire stack.
        #[builder(into, default)]
        pub custom_json: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the availability zone where instances will be created by default.
        /// Cannot be set when `vpc_id` is set.
        #[builder(into, default)]
        pub default_availability_zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ARN of an IAM Instance Profile that created instances will have by default.
        #[builder(into)]
        pub default_instance_profile_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of OS that will be installed on instances by default.
        #[builder(into, default)]
        pub default_os: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the type of root device instances will have by default.
        #[builder(into, default)]
        pub default_root_device_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the SSH keypair that instances will have by default.
        #[builder(into, default)]
        pub default_ssh_key_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ID of the subnet in which instances will be created by default.
        /// Required if `vpc_id` is set to a VPC other than the default VPC, and forbidden if it isn't.
        #[builder(into, default)]
        pub default_subnet_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Keyword representing the naming scheme that will be used for instance hostnames within this stack.
        #[builder(into, default)]
        pub hostname_theme: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Boolean value controlling whether Opsworks will run Berkshelf for this stack.
        #[builder(into, default)]
        pub manage_berkshelf: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name of the stack.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the region where the stack will exist.
        #[builder(into)]
        pub region: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ARN of an IAM role that the OpsWorks service will act as.
        #[builder(into)]
        pub service_role_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource.
        /// If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Boolean value controlling whether the custom cookbook settings are enabled.
        #[builder(into, default)]
        pub use_custom_cookbooks: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Boolean value controlling whether the standard OpsWorks security groups apply to created instances.
        #[builder(into, default)]
        pub use_opsworks_security_groups: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// ID of the VPC that this stack belongs to.
        /// Defaults to the region's default VPC.
        #[builder(into, default)]
        pub vpc_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct StackResult {
        /// If set to `"LATEST"`, OpsWorks will automatically install the latest version.
        pub agent_version: pulumi_wasm_rust::Output<String>,
        pub arn: pulumi_wasm_rust::Output<String>,
        /// If `manage_berkshelf` is enabled, the version of Berkshelf to use.
        pub berkshelf_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Color to paint next to the stack's resources in the OpsWorks console.
        pub color: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the configuration manager to use. Defaults to "Chef".
        pub configuration_manager_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Version of the configuration manager to use. Defaults to "11.4".
        pub configuration_manager_version: pulumi_wasm_rust::Output<Option<String>>,
        /// When `use_custom_cookbooks` is set, provide this sub-object as described below.
        pub custom_cookbooks_sources: pulumi_wasm_rust::Output<
            Vec<super::super::types::opsworks::StackCustomCookbooksSource>,
        >,
        /// Custom JSON attributes to apply to the entire stack.
        pub custom_json: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the availability zone where instances will be created by default.
        /// Cannot be set when `vpc_id` is set.
        pub default_availability_zone: pulumi_wasm_rust::Output<String>,
        /// The ARN of an IAM Instance Profile that created instances will have by default.
        pub default_instance_profile_arn: pulumi_wasm_rust::Output<String>,
        /// Name of OS that will be installed on instances by default.
        pub default_os: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the type of root device instances will have by default.
        pub default_root_device_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the SSH keypair that instances will have by default.
        pub default_ssh_key_name: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the subnet in which instances will be created by default.
        /// Required if `vpc_id` is set to a VPC other than the default VPC, and forbidden if it isn't.
        pub default_subnet_id: pulumi_wasm_rust::Output<String>,
        /// Keyword representing the naming scheme that will be used for instance hostnames within this stack.
        pub hostname_theme: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean value controlling whether Opsworks will run Berkshelf for this stack.
        pub manage_berkshelf: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the stack.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the region where the stack will exist.
        pub region: pulumi_wasm_rust::Output<String>,
        /// The ARN of an IAM role that the OpsWorks service will act as.
        pub service_role_arn: pulumi_wasm_rust::Output<String>,
        pub stack_endpoint: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource.
        /// If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Boolean value controlling whether the custom cookbook settings are enabled.
        pub use_custom_cookbooks: pulumi_wasm_rust::Output<Option<bool>>,
        /// Boolean value controlling whether the standard OpsWorks security groups apply to created instances.
        pub use_opsworks_security_groups: pulumi_wasm_rust::Output<Option<bool>>,
        /// ID of the VPC that this stack belongs to.
        /// Defaults to the region's default VPC.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: StackArgs,
    ) -> StackResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let agent_version_binding = args.agent_version.get_output(context).get_inner();
        let berkshelf_version_binding = args
            .berkshelf_version
            .get_output(context)
            .get_inner();
        let color_binding = args.color.get_output(context).get_inner();
        let configuration_manager_name_binding = args
            .configuration_manager_name
            .get_output(context)
            .get_inner();
        let configuration_manager_version_binding = args
            .configuration_manager_version
            .get_output(context)
            .get_inner();
        let custom_cookbooks_sources_binding = args
            .custom_cookbooks_sources
            .get_output(context)
            .get_inner();
        let custom_json_binding = args.custom_json.get_output(context).get_inner();
        let default_availability_zone_binding = args
            .default_availability_zone
            .get_output(context)
            .get_inner();
        let default_instance_profile_arn_binding = args
            .default_instance_profile_arn
            .get_output(context)
            .get_inner();
        let default_os_binding = args.default_os.get_output(context).get_inner();
        let default_root_device_type_binding = args
            .default_root_device_type
            .get_output(context)
            .get_inner();
        let default_ssh_key_name_binding = args
            .default_ssh_key_name
            .get_output(context)
            .get_inner();
        let default_subnet_id_binding = args
            .default_subnet_id
            .get_output(context)
            .get_inner();
        let hostname_theme_binding = args.hostname_theme.get_output(context).get_inner();
        let manage_berkshelf_binding = args
            .manage_berkshelf
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let service_role_arn_binding = args
            .service_role_arn
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let use_custom_cookbooks_binding = args
            .use_custom_cookbooks
            .get_output(context)
            .get_inner();
        let use_opsworks_security_groups_binding = args
            .use_opsworks_security_groups
            .get_output(context)
            .get_inner();
        let vpc_id_binding = args.vpc_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opsworks/stack:Stack".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "agentVersion".into(),
                    value: &agent_version_binding,
                },
                register_interface::ObjectField {
                    name: "berkshelfVersion".into(),
                    value: &berkshelf_version_binding,
                },
                register_interface::ObjectField {
                    name: "color".into(),
                    value: &color_binding,
                },
                register_interface::ObjectField {
                    name: "configurationManagerName".into(),
                    value: &configuration_manager_name_binding,
                },
                register_interface::ObjectField {
                    name: "configurationManagerVersion".into(),
                    value: &configuration_manager_version_binding,
                },
                register_interface::ObjectField {
                    name: "customCookbooksSources".into(),
                    value: &custom_cookbooks_sources_binding,
                },
                register_interface::ObjectField {
                    name: "customJson".into(),
                    value: &custom_json_binding,
                },
                register_interface::ObjectField {
                    name: "defaultAvailabilityZone".into(),
                    value: &default_availability_zone_binding,
                },
                register_interface::ObjectField {
                    name: "defaultInstanceProfileArn".into(),
                    value: &default_instance_profile_arn_binding,
                },
                register_interface::ObjectField {
                    name: "defaultOs".into(),
                    value: &default_os_binding,
                },
                register_interface::ObjectField {
                    name: "defaultRootDeviceType".into(),
                    value: &default_root_device_type_binding,
                },
                register_interface::ObjectField {
                    name: "defaultSshKeyName".into(),
                    value: &default_ssh_key_name_binding,
                },
                register_interface::ObjectField {
                    name: "defaultSubnetId".into(),
                    value: &default_subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "hostnameTheme".into(),
                    value: &hostname_theme_binding,
                },
                register_interface::ObjectField {
                    name: "manageBerkshelf".into(),
                    value: &manage_berkshelf_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
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
                    name: "useCustomCookbooks".into(),
                    value: &use_custom_cookbooks_binding,
                },
                register_interface::ObjectField {
                    name: "useOpsworksSecurityGroups".into(),
                    value: &use_opsworks_security_groups_binding,
                },
                register_interface::ObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "agentVersion".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "berkshelfVersion".into(),
                },
                register_interface::ResultField {
                    name: "color".into(),
                },
                register_interface::ResultField {
                    name: "configurationManagerName".into(),
                },
                register_interface::ResultField {
                    name: "configurationManagerVersion".into(),
                },
                register_interface::ResultField {
                    name: "customCookbooksSources".into(),
                },
                register_interface::ResultField {
                    name: "customJson".into(),
                },
                register_interface::ResultField {
                    name: "defaultAvailabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "defaultInstanceProfileArn".into(),
                },
                register_interface::ResultField {
                    name: "defaultOs".into(),
                },
                register_interface::ResultField {
                    name: "defaultRootDeviceType".into(),
                },
                register_interface::ResultField {
                    name: "defaultSshKeyName".into(),
                },
                register_interface::ResultField {
                    name: "defaultSubnetId".into(),
                },
                register_interface::ResultField {
                    name: "hostnameTheme".into(),
                },
                register_interface::ResultField {
                    name: "manageBerkshelf".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "serviceRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "stackEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "useCustomCookbooks".into(),
                },
                register_interface::ResultField {
                    name: "useOpsworksSecurityGroups".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        StackResult {
            agent_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agentVersion").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            berkshelf_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("berkshelfVersion").unwrap(),
            ),
            color: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("color").unwrap(),
            ),
            configuration_manager_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationManagerName").unwrap(),
            ),
            configuration_manager_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationManagerVersion").unwrap(),
            ),
            custom_cookbooks_sources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customCookbooksSources").unwrap(),
            ),
            custom_json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customJson").unwrap(),
            ),
            default_availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultAvailabilityZone").unwrap(),
            ),
            default_instance_profile_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultInstanceProfileArn").unwrap(),
            ),
            default_os: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultOs").unwrap(),
            ),
            default_root_device_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultRootDeviceType").unwrap(),
            ),
            default_ssh_key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSshKeyName").unwrap(),
            ),
            default_subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSubnetId").unwrap(),
            ),
            hostname_theme: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostnameTheme").unwrap(),
            ),
            manage_berkshelf: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("manageBerkshelf").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            service_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceRoleArn").unwrap(),
            ),
            stack_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stackEndpoint").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            use_custom_cookbooks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("useCustomCookbooks").unwrap(),
            ),
            use_opsworks_security_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("useOpsworksSecurityGroups").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}
