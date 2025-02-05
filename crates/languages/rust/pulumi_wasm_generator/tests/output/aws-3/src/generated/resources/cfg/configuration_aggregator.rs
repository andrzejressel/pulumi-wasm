/// Manages an AWS Config Configuration Aggregator
///
/// ## Example Usage
///
/// ### Account Based Aggregation
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod configuration_aggregator {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationAggregatorArgs {
        /// The account(s) to aggregate config data from as documented below.
        #[builder(into, default)]
        pub account_aggregation_source: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::cfg::ConfigurationAggregatorAccountAggregationSource,
            >,
        >,
        /// The name of the configuration aggregator.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The organization to aggregate config data from as documented below.
        #[builder(into, default)]
        pub organization_aggregation_source: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::cfg::ConfigurationAggregatorOrganizationAggregationSource,
            >,
        >,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// Either `account_aggregation_source` or `organization_aggregation_source` must be specified.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConfigurationAggregatorResult {
        /// The account(s) to aggregate config data from as documented below.
        pub account_aggregation_source: pulumi_wasm_rust::Output<
            Option<
                super::super::types::cfg::ConfigurationAggregatorAccountAggregationSource,
            >,
        >,
        /// The ARN of the aggregator
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name of the configuration aggregator.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The organization to aggregate config data from as documented below.
        pub organization_aggregation_source: pulumi_wasm_rust::Output<
            Option<
                super::super::types::cfg::ConfigurationAggregatorOrganizationAggregationSource,
            >,
        >,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// Either `account_aggregation_source` or `organization_aggregation_source` must be specified.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ConfigurationAggregatorArgs,
    ) -> ConfigurationAggregatorResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_aggregation_source_binding = args
            .account_aggregation_source
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let organization_aggregation_source_binding = args
            .organization_aggregation_source
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cfg/configurationAggregator:ConfigurationAggregator".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountAggregationSource".into(),
                    value: &account_aggregation_source_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "organizationAggregationSource".into(),
                    value: &organization_aggregation_source_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ConfigurationAggregatorResult {
            account_aggregation_source: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountAggregationSource"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            organization_aggregation_source: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("organizationAggregationSource"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
