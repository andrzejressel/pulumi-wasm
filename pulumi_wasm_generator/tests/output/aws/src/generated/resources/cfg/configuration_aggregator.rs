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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationAggregatorArgs {
        /// The account(s) to aggregate config data from as documented below.
        #[builder(into, default)]
        pub account_aggregation_source: pulumi_wasm_rust::Output<
            Option<
                super::super::types::cfg::ConfigurationAggregatorAccountAggregationSource,
            >,
        >,
        /// The name of the configuration aggregator.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The organization to aggregate config data from as documented below.
        #[builder(into, default)]
        pub organization_aggregation_source: pulumi_wasm_rust::Output<
            Option<
                super::super::types::cfg::ConfigurationAggregatorOrganizationAggregationSource,
            >,
        >,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// Either `account_aggregation_source` or `organization_aggregation_source` must be specified.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
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
        name: &str,
        args: ConfigurationAggregatorArgs,
    ) -> ConfigurationAggregatorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_aggregation_source_binding = args
            .account_aggregation_source
            .get_inner();
        let name_binding = args.name.get_inner();
        let organization_aggregation_source_binding = args
            .organization_aggregation_source
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cfg/configurationAggregator:ConfigurationAggregator".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountAggregationSource".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "organizationAggregationSource".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConfigurationAggregatorResult {
            account_aggregation_source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountAggregationSource").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            organization_aggregation_source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("organizationAggregationSource").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
