/// Manages a Palo Alto Local Rulestack Rule.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("rg-example")
///             .build_struct(),
///     );
///     let exampleLocalRulestack = local_rulestack::create(
///         "exampleLocalRulestack",
///         LocalRulestackArgs::builder()
///             .location("${example.location}")
///             .name("lrs-example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleLocalRulestackRule = local_rulestack_rule::create(
///         "exampleLocalRulestackRule",
///         LocalRulestackRuleArgs::builder()
///             .action("Allow")
///             .applications(vec!["any",])
///             .destination(
///                 LocalRulestackRuleDestination::builder()
///                     .cidrs(vec!["192.168.16.0/24",])
///                     .build_struct(),
///             )
///             .name("example-rule")
///             .priority(1000)
///             .protocol("application-default")
///             .rulestack_id("${exampleLocalRulestack.id}")
///             .source(
///                 LocalRulestackRuleSource::builder()
///                     .cidrs(vec!["10.0.0.0/8",])
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Palo Alto Local Rulestack Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:paloalto/localRulestackRule:LocalRulestackRule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/PaloAltoNetworks.Cloudngfw/localRulestacks/myLocalRulestack/localRules/myRule1
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod local_rulestack_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocalRulestackRuleArgs {
        /// The action to take on the rule being triggered. Possible values are `Allow`, `DenyResetBoth`, `DenyResetServer` and `DenySilent`.
        #[builder(into)]
        pub action: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies a list of Applications.
        #[builder(into)]
        pub applications: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The comment for Audit purposes.
        #[builder(into, default)]
        pub audit_comment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `category` block as defined below.
        #[builder(into, default)]
        pub category: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::paloalto::LocalRulestackRuleCategory>,
        >,
        /// The type of Decryption to perform on the rule. Possible values include `SSLInboundInspection`, `SSLOutboundInspection`, and `None`. Defaults to `None`.
        #[builder(into, default)]
        pub decryption_rule_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description for the rule.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `destination` blocks as defined below.
        #[builder(into)]
        pub destination: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::paloalto::LocalRulestackRuleDestination,
        >,
        /// Should this Rule be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ID of the certificate for inbound inspection. Only valid when `decryption_rule_type` is set to `SSLInboundInspection`.
        #[builder(into, default)]
        pub inspection_certificate_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Should Logging be enabled? Defaults to `false`.
        #[builder(into, default)]
        pub logging_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name which should be used for this Palo Alto Local Rulestack Rule.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should the inverse of the Destination configuration be used. Defaults to `false`.
        #[builder(into, default)]
        pub negate_destination: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Should the inverse of the Source configuration be used. Defaults to `false`.
        #[builder(into, default)]
        pub negate_source: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Priority of this rule. Rules are executed in numerical order. Changing this forces a new Palo Alto Local Rulestack Rule to be created.
        ///
        /// > **NOTE:** This is the primary identifier of a rule, as such it is not possible to change the Priority of a rule once created.
        #[builder(into)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The Protocol and port to use in the form `[protocol]:[port_number]` e.g. `TCP:8080` or `UDP:53`. Conflicts with `protocol_ports`. Defaults to `application-default`.
        ///
        /// > **NOTE** In 4.0 or later versions, the default of `protocol` will no longer be set by provider, exactly one of `protocol` and `protocol_ports` must be specified. You need to explicitly specify `protocol="application-default"` to keep the the current default of the `protocol`.
        #[builder(into, default)]
        pub protocol: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies a list of Protocol:Port entries. E.g. `[ "TCP:80", "UDP:5431" ]`. Conflicts with `protocol`.
        #[builder(into, default)]
        pub protocol_ports: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ID of the Local Rulestack in which to create this Rule. Changing this forces a new Palo Alto Local Rulestack Rule to be created.
        #[builder(into)]
        pub rulestack_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `source` blocks as defined below.
        #[builder(into)]
        pub source: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::paloalto::LocalRulestackRuleSource,
        >,
        /// A mapping of tags which should be assigned to the Palo Alto Local Rulestack Rule.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LocalRulestackRuleResult {
        /// The action to take on the rule being triggered. Possible values are `Allow`, `DenyResetBoth`, `DenyResetServer` and `DenySilent`.
        pub action: pulumi_gestalt_rust::Output<String>,
        /// Specifies a list of Applications.
        pub applications: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The comment for Audit purposes.
        pub audit_comment: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `category` block as defined below.
        pub category: pulumi_gestalt_rust::Output<
            Option<super::super::types::paloalto::LocalRulestackRuleCategory>,
        >,
        /// The type of Decryption to perform on the rule. Possible values include `SSLInboundInspection`, `SSLOutboundInspection`, and `None`. Defaults to `None`.
        pub decryption_rule_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The description for the rule.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more `destination` blocks as defined below.
        pub destination: pulumi_gestalt_rust::Output<
            super::super::types::paloalto::LocalRulestackRuleDestination,
        >,
        /// Should this Rule be enabled? Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the certificate for inbound inspection. Only valid when `decryption_rule_type` is set to `SSLInboundInspection`.
        pub inspection_certificate_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Should Logging be enabled? Defaults to `false`.
        pub logging_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name which should be used for this Palo Alto Local Rulestack Rule.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Should the inverse of the Destination configuration be used. Defaults to `false`.
        pub negate_destination: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Should the inverse of the Source configuration be used. Defaults to `false`.
        pub negate_source: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Priority of this rule. Rules are executed in numerical order. Changing this forces a new Palo Alto Local Rulestack Rule to be created.
        ///
        /// > **NOTE:** This is the primary identifier of a rule, as such it is not possible to change the Priority of a rule once created.
        pub priority: pulumi_gestalt_rust::Output<i32>,
        /// The Protocol and port to use in the form `[protocol]:[port_number]` e.g. `TCP:8080` or `UDP:53`. Conflicts with `protocol_ports`. Defaults to `application-default`.
        ///
        /// > **NOTE** In 4.0 or later versions, the default of `protocol` will no longer be set by provider, exactly one of `protocol` and `protocol_ports` must be specified. You need to explicitly specify `protocol="application-default"` to keep the the current default of the `protocol`.
        pub protocol: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies a list of Protocol:Port entries. E.g. `[ "TCP:80", "UDP:5431" ]`. Conflicts with `protocol`.
        pub protocol_ports: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The ID of the Local Rulestack in which to create this Rule. Changing this forces a new Palo Alto Local Rulestack Rule to be created.
        pub rulestack_id: pulumi_gestalt_rust::Output<String>,
        /// One or more `source` blocks as defined below.
        pub source: pulumi_gestalt_rust::Output<
            super::super::types::paloalto::LocalRulestackRuleSource,
        >,
        /// A mapping of tags which should be assigned to the Palo Alto Local Rulestack Rule.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: LocalRulestackRuleArgs,
    ) -> LocalRulestackRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_output(context).get_inner();
        let applications_binding = args.applications.get_output(context).get_inner();
        let audit_comment_binding = args.audit_comment.get_output(context).get_inner();
        let category_binding = args.category.get_output(context).get_inner();
        let decryption_rule_type_binding = args
            .decryption_rule_type
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let destination_binding = args.destination.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let inspection_certificate_id_binding = args
            .inspection_certificate_id
            .get_output(context)
            .get_inner();
        let logging_enabled_binding = args
            .logging_enabled
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let negate_destination_binding = args
            .negate_destination
            .get_output(context)
            .get_inner();
        let negate_source_binding = args.negate_source.get_output(context).get_inner();
        let priority_binding = args.priority.get_output(context).get_inner();
        let protocol_binding = args.protocol.get_output(context).get_inner();
        let protocol_ports_binding = args.protocol_ports.get_output(context).get_inner();
        let rulestack_id_binding = args.rulestack_id.get_output(context).get_inner();
        let source_binding = args.source.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:paloalto/localRulestackRule:LocalRulestackRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "action".into(),
                    value: &action_binding,
                },
                register_interface::ObjectField {
                    name: "applications".into(),
                    value: &applications_binding,
                },
                register_interface::ObjectField {
                    name: "auditComment".into(),
                    value: &audit_comment_binding,
                },
                register_interface::ObjectField {
                    name: "category".into(),
                    value: &category_binding,
                },
                register_interface::ObjectField {
                    name: "decryptionRuleType".into(),
                    value: &decryption_rule_type_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "destination".into(),
                    value: &destination_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "inspectionCertificateId".into(),
                    value: &inspection_certificate_id_binding,
                },
                register_interface::ObjectField {
                    name: "loggingEnabled".into(),
                    value: &logging_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "negateDestination".into(),
                    value: &negate_destination_binding,
                },
                register_interface::ObjectField {
                    name: "negateSource".into(),
                    value: &negate_source_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding,
                },
                register_interface::ObjectField {
                    name: "protocolPorts".into(),
                    value: &protocol_ports_binding,
                },
                register_interface::ObjectField {
                    name: "rulestackId".into(),
                    value: &rulestack_id_binding,
                },
                register_interface::ObjectField {
                    name: "source".into(),
                    value: &source_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LocalRulestackRuleResult {
            action: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("action"),
            ),
            applications: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applications"),
            ),
            audit_comment: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("auditComment"),
            ),
            category: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("category"),
            ),
            decryption_rule_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("decryptionRuleType"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            destination: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destination"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            inspection_certificate_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inspectionCertificateId"),
            ),
            logging_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loggingEnabled"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            negate_destination: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("negateDestination"),
            ),
            negate_source: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("negateSource"),
            ),
            priority: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("priority"),
            ),
            protocol: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("protocol"),
            ),
            protocol_ports: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("protocolPorts"),
            ),
            rulestack_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rulestackId"),
            ),
            source: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("source"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
