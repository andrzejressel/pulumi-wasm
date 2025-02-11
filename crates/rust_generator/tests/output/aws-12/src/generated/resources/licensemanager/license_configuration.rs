/// Provides a License Manager license configuration resource.
///
/// > **Note:** Removing the `license_count` attribute is not supported by the License Manager API.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:licensemanager:LicenseConfiguration
///     properties:
///       name: Example
///       description: Example
///       licenseCount: 10
///       licenseCountHardLimit: true
///       licenseCountingType: Socket
///       licenseRules:
///         - '#minimumSockets=2'
///       tags:
///         foo: barr
/// ```
///
/// ## Rules
///
/// License rules should be in the format of `#RuleType=RuleValue`. Supported rule types:
///
/// * `minimumVcpus` - Resource must have minimum vCPU count in order to use the license. Default: 1
/// * `maximumVcpus` - Resource must have maximum vCPU count in order to use the license. Default: unbounded, limit: 10000
/// * `minimumCores` - Resource must have minimum core count in order to use the license. Default: 1
/// * `maximumCores` - Resource must have maximum core count in order to use the license. Default: unbounded, limit: 10000
/// * `minimumSockets` - Resource must have minimum socket count in order to use the license. Default: 1
/// * `maximumSockets` - Resource must have maximum socket count in order to use the license. Default: unbounded, limit: 10000
/// * `allowedTenancy` - Defines where the license can be used. If set, restricts license usage to selected tenancies. Specify a comma delimited list of `EC2-Default`, `EC2-DedicatedHost`, `EC2-DedicatedInstance`
///
/// ## Import
///
/// Using `pulumi import`, import license configurations using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:licensemanager/licenseConfiguration:LicenseConfiguration example arn:aws:license-manager:eu-west-1:123456789012:license-configuration:lic-0123456789abcdef0123456789abcdef
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod license_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LicenseConfigurationArgs {
        /// Description of the license configuration.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Number of licenses managed by the license configuration.
        #[builder(into, default)]
        pub license_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Sets the number of available licenses as a hard limit.
        #[builder(into, default)]
        pub license_count_hard_limit: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Dimension to use to track license inventory. Specify either `vCPU`, `Instance`, `Core` or `Socket`.
        #[builder(into)]
        pub license_counting_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Array of configured License Manager rules.
        #[builder(into, default)]
        pub license_rules: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Name of the license configuration.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LicenseConfigurationResult {
        /// The license configuration ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the license configuration.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Number of licenses managed by the license configuration.
        pub license_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Sets the number of available licenses as a hard limit.
        pub license_count_hard_limit: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Dimension to use to track license inventory. Specify either `vCPU`, `Instance`, `Core` or `Socket`.
        pub license_counting_type: pulumi_gestalt_rust::Output<String>,
        /// Array of configured License Manager rules.
        pub license_rules: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Name of the license configuration.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Account ID of the owner of the license configuration.
        pub owner_account_id: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LicenseConfigurationArgs,
    ) -> LicenseConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let license_count_binding = args.license_count.get_output(context);
        let license_count_hard_limit_binding = args
            .license_count_hard_limit
            .get_output(context);
        let license_counting_type_binding = args
            .license_counting_type
            .get_output(context);
        let license_rules_binding = args.license_rules.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:licensemanager/licenseConfiguration:LicenseConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "licenseCount".into(),
                    value: &license_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "licenseCountHardLimit".into(),
                    value: &license_count_hard_limit_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "licenseCountingType".into(),
                    value: &license_counting_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "licenseRules".into(),
                    value: &license_rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LicenseConfigurationResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            license_count: o.get_field("licenseCount"),
            license_count_hard_limit: o.get_field("licenseCountHardLimit"),
            license_counting_type: o.get_field("licenseCountingType"),
            license_rules: o.get_field("licenseRules"),
            name: o.get_field("name"),
            owner_account_id: o.get_field("ownerAccountId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
