/// Manages an AWS Config Configuration Aggregator
///
/// ## Example Usage
///
/// ### Account Based Aggregation
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let account = configuration_aggregator::create(
///         "account",
///         ConfigurationAggregatorArgs::builder()
///             .account_aggregation_source(
///                 ConfigurationAggregatorAccountAggregationSource::builder()
///                     .accountIds(vec!["123456789012",])
///                     .regions(vec!["us-west-2",])
///                     .build_struct(),
///             )
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Organization Based Aggregation
///
/// ```yaml
/// resources:
///   organization:
///     type: aws:cfg:ConfigurationAggregator
///     properties:
///       name: example
///       organizationAggregationSource:
///         allRegions: true
///         roleArn: ${organizationRole.arn}
///     options:
///       dependsOn:
///         - ${organizationRolePolicyAttachment}
///   organizationRole:
///     type: aws:iam:Role
///     name: organization
///     properties:
///       name: example
///       assumeRolePolicy: ${assumeRole.json}
///   organizationRolePolicyAttachment:
///     type: aws:iam:RolePolicyAttachment
///     name: organization
///     properties:
///       role: ${organizationRole.name}
///       policyArn: arn:aws:iam::aws:policy/service-role/AWSConfigRoleForOrganizations
/// variables:
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - config.amazonaws.com
///             actions:
///               - sts:AssumeRole
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Configuration Aggregators using the name. For example:
///
/// ```sh
/// $ pulumi import aws:cfg/configurationAggregator:ConfigurationAggregator example foo
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod configuration_aggregator {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationAggregatorArgs {
        /// The account(s) to aggregate config data from as documented below.
        #[builder(into, default)]
        pub account_aggregation_source: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::cfg::ConfigurationAggregatorAccountAggregationSource,
            >,
        >,
        /// The name of the configuration aggregator.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The organization to aggregate config data from as documented below.
        #[builder(into, default)]
        pub organization_aggregation_source: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::cfg::ConfigurationAggregatorOrganizationAggregationSource,
            >,
        >,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// Either `account_aggregation_source` or `organization_aggregation_source` must be specified.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConfigurationAggregatorResult {
        /// The account(s) to aggregate config data from as documented below.
        pub account_aggregation_source: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::cfg::ConfigurationAggregatorAccountAggregationSource,
            >,
        >,
        /// The ARN of the aggregator
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the configuration aggregator.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The organization to aggregate config data from as documented below.
        pub organization_aggregation_source: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::cfg::ConfigurationAggregatorOrganizationAggregationSource,
            >,
        >,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// Either `account_aggregation_source` or `organization_aggregation_source` must be specified.
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
        args: ConfigurationAggregatorArgs,
    ) -> ConfigurationAggregatorResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_aggregation_source_binding = args
            .account_aggregation_source
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let organization_aggregation_source_binding = args
            .organization_aggregation_source
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cfg/configurationAggregator:ConfigurationAggregator".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountAggregationSource".into(),
                    value: &account_aggregation_source_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "organizationAggregationSource".into(),
                    value: &organization_aggregation_source_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConfigurationAggregatorResult {
            account_aggregation_source: o.get_field("accountAggregationSource"),
            arn: o.get_field("arn"),
            name: o.get_field("name"),
            organization_aggregation_source: o
                .get_field("organizationAggregationSource"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
