/// Resource for managing an AWS EMR block public access configuration. This region level security configuration restricts the launch of EMR clusters that have associated security groups permitting public access on unspecified ports. See the [EMR Block Public Access Configuration](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-block-public-access.html) documentation for further information.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod block_public_access_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BlockPublicAccessConfigurationArgs {
        /// Enable or disable EMR Block Public Access.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub block_public_security_group_rules: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// Configuration block for defining permitted public security group rule port ranges. Can be defined multiple times per resource. Only valid if `block_public_security_group_rules` is set to `true`.
        #[builder(into, default)]
        pub permitted_public_security_group_rule_ranges: pulumi_gestalt_rust::InputOrOutput<
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
        pub block_public_security_group_rules: pulumi_gestalt_rust::Output<bool>,
        /// Configuration block for defining permitted public security group rule port ranges. Can be defined multiple times per resource. Only valid if `block_public_security_group_rules` is set to `true`.
        pub permitted_public_security_group_rule_ranges: pulumi_gestalt_rust::Output<
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BlockPublicAccessConfigurationArgs,
    ) -> BlockPublicAccessConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let block_public_security_group_rules_binding = args
            .block_public_security_group_rules
            .get_output(context);
        let permitted_public_security_group_rule_ranges_binding = args
            .permitted_public_security_group_rule_ranges
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:emr/blockPublicAccessConfiguration:BlockPublicAccessConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blockPublicSecurityGroupRules".into(),
                    value: &block_public_security_group_rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "permittedPublicSecurityGroupRuleRanges".into(),
                    value: &permitted_public_security_group_rule_ranges_binding
                        .drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BlockPublicAccessConfigurationResult {
            block_public_security_group_rules: o
                .get_field("blockPublicSecurityGroupRules"),
            permitted_public_security_group_rule_ranges: o
                .get_field("permittedPublicSecurityGroupRuleRanges"),
        }
    }
}
