/// Resource for managing an AWS EMR block public access configuration. This region level security configuration restricts the launch of EMR clusters that have associated security groups permitting public access on unspecified ports. See the [EMR Block Public Access Configuration](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-block-public-access.html) documentation for further information.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = block_public_access_configuration::create(
///         "example",
///         BlockPublicAccessConfigurationArgs::builder()
///             .block_public_security_group_rules(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Default Configuration
///
/// By default, each AWS region is equipped with a block public access configuration that prevents EMR clusters from being launched if they have security group rules permitting public access on any port except for port 22. The default configuration can be managed using this resource.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = block_public_access_configuration::create(
///         "example",
///         BlockPublicAccessConfigurationArgs::builder()
///             .block_public_security_group_rules(true)
///             .permitted_public_security_group_rule_ranges(
///                 vec![
///                     BlockPublicAccessConfigurationPermittedPublicSecurityGroupRuleRange::builder()
///                     .maxRange(22).minRange(22).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// > **NOTE:** If an `aws.emr.BlockPublicAccessConfiguration` resource is destroyed, the configuration will reset to this default configuration.
///
/// ### Multiple Permitted Public Security Group Rule Ranges
///
/// The resource permits specification of multiple `permitted_public_security_group_rule_range` blocks.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = block_public_access_configuration::create(
///         "example",
///         BlockPublicAccessConfigurationArgs::builder()
///             .block_public_security_group_rules(true)
///             .permitted_public_security_group_rule_ranges(
///                 vec![
///                     BlockPublicAccessConfigurationPermittedPublicSecurityGroupRuleRange::builder()
///                     .maxRange(22).minRange(22).build_struct(),
///                     BlockPublicAccessConfigurationPermittedPublicSecurityGroupRuleRange::builder()
///                     .maxRange(101).minRange(100).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Disabling Block Public Access
///
/// To permit EMR clusters to be launched in the configured region regardless of associated security group rules, the Block Public Access feature can be disabled using this resource.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = block_public_access_configuration::create(
///         "example",
///         BlockPublicAccessConfigurationArgs::builder()
///             .block_public_security_group_rules(false)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the current EMR Block Public Access Configuration. For example:
///
/// ```sh
/// $ pulumi import aws:emr/blockPublicAccessConfiguration:BlockPublicAccessConfiguration example current
/// ```
pub mod block_public_access_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BlockPublicAccessConfigurationArgs {
        /// Enable or disable EMR Block Public Access.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub block_public_security_group_rules: pulumi_wasm_rust::InputOrOutput<bool>,
        /// Configuration block for defining permitted public security group rule port ranges. Can be defined multiple times per resource. Only valid if `block_public_security_group_rules` is set to `true`.
        #[builder(into, default)]
        pub permitted_public_security_group_rule_ranges: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::emr::BlockPublicAccessConfigurationPermittedPublicSecurityGroupRuleRange,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct BlockPublicAccessConfigurationResult {
        /// Enable or disable EMR Block Public Access.
        ///
        /// The following arguments are optional:
        pub block_public_security_group_rules: pulumi_wasm_rust::Output<bool>,
        /// Configuration block for defining permitted public security group rule port ranges. Can be defined multiple times per resource. Only valid if `block_public_security_group_rules` is set to `true`.
        pub permitted_public_security_group_rule_ranges: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::emr::BlockPublicAccessConfigurationPermittedPublicSecurityGroupRuleRange,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: BlockPublicAccessConfigurationArgs,
    ) -> BlockPublicAccessConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let block_public_security_group_rules_binding = args
            .block_public_security_group_rules
            .get_output(context)
            .get_inner();
        let permitted_public_security_group_rule_ranges_binding = args
            .permitted_public_security_group_rule_ranges
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:emr/blockPublicAccessConfiguration:BlockPublicAccessConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "blockPublicSecurityGroupRules".into(),
                    value: &block_public_security_group_rules_binding,
                },
                register_interface::ObjectField {
                    name: "permittedPublicSecurityGroupRuleRanges".into(),
                    value: &permitted_public_security_group_rule_ranges_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BlockPublicAccessConfigurationResult {
            block_public_security_group_rules: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("blockPublicSecurityGroupRules"),
            ),
            permitted_public_security_group_rule_ranges: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("permittedPublicSecurityGroupRuleRanges"),
            ),
        }
    }
}
