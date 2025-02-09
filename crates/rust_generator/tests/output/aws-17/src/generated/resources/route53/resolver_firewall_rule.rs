/// Provides a Route 53 Resolver DNS Firewall rule resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:route53:ResolverFirewallDomainList
///     properties:
///       name: example
///       domains:
///         - example.com
///       tags: {}
///   exampleResolverFirewallRuleGroup:
///     type: aws:route53:ResolverFirewallRuleGroup
///     name: example
///     properties:
///       name: example
///       tags: {}
///   exampleResolverFirewallRule:
///     type: aws:route53:ResolverFirewallRule
///     name: example
///     properties:
///       name: example
///       action: BLOCK
///       blockOverrideDnsType: CNAME
///       blockOverrideDomain: example.com
///       blockOverrideTtl: 1
///       blockResponse: OVERRIDE
///       firewallDomainListId: ${example.id}
///       firewallRuleGroupId: ${exampleResolverFirewallRuleGroup.id}
///       priority: 100
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import  Route 53 Resolver DNS Firewall rules using the Route 53 Resolver DNS Firewall rule group ID and domain list ID separated by ':'. For example:
///
/// ```sh
/// $ pulumi import aws:route53/resolverFirewallRule:ResolverFirewallRule example rslvr-frg-0123456789abcdef:rslvr-fdl-0123456789abcdef
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resolver_firewall_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResolverFirewallRuleArgs {
        /// The action that DNS Firewall should take on a DNS query when it matches one of the domains in the rule's domain list. Valid values: `ALLOW`, `BLOCK`, `ALERT`.
        #[builder(into)]
        pub action: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The DNS record's type. This determines the format of the record value that you provided in BlockOverrideDomain. Value values: `CNAME`.
        #[builder(into, default)]
        pub block_override_dns_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The custom DNS record to send back in response to the query.
        #[builder(into, default)]
        pub block_override_domain: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The recommended amount of time, in seconds, for the DNS resolver or web browser to cache the provided override record. Minimum value of 0. Maximum value of 604800.
        #[builder(into, default)]
        pub block_override_ttl: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The way that you want DNS Firewall to block the request. Valid values: `NODATA`, `NXDOMAIN`, `OVERRIDE`.
        #[builder(into, default)]
        pub block_response: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the domain list that you want to use in the rule.
        #[builder(into)]
        pub firewall_domain_list_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Evaluate DNS redirection in the DNS redirection chain, such as CNAME, DNAME, ot ALIAS. Valid values are `INSPECT_REDIRECTION_DOMAIN` and `TRUST_REDIRECTION_DOMAIN`. Default value is `INSPECT_REDIRECTION_DOMAIN`.
        #[builder(into, default)]
        pub firewall_domain_redirection_action: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The unique identifier of the firewall rule group where you want to create the rule.
        #[builder(into)]
        pub firewall_rule_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A name that lets you identify the rule, to manage and use it.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The setting that determines the processing order of the rule in the rule group. DNS Firewall processes the rules in a rule group by order of priority, starting from the lowest setting.
        #[builder(into)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The query type you want the rule to evaluate. Additional details can be found [here](https://en.wikipedia.org/wiki/List_of_DNS_record_types)
        #[builder(into, default)]
        pub q_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ResolverFirewallRuleResult {
        /// The action that DNS Firewall should take on a DNS query when it matches one of the domains in the rule's domain list. Valid values: `ALLOW`, `BLOCK`, `ALERT`.
        pub action: pulumi_gestalt_rust::Output<String>,
        /// The DNS record's type. This determines the format of the record value that you provided in BlockOverrideDomain. Value values: `CNAME`.
        pub block_override_dns_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The custom DNS record to send back in response to the query.
        pub block_override_domain: pulumi_gestalt_rust::Output<Option<String>>,
        /// The recommended amount of time, in seconds, for the DNS resolver or web browser to cache the provided override record. Minimum value of 0. Maximum value of 604800.
        pub block_override_ttl: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The way that you want DNS Firewall to block the request. Valid values: `NODATA`, `NXDOMAIN`, `OVERRIDE`.
        pub block_response: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the domain list that you want to use in the rule.
        pub firewall_domain_list_id: pulumi_gestalt_rust::Output<String>,
        /// Evaluate DNS redirection in the DNS redirection chain, such as CNAME, DNAME, ot ALIAS. Valid values are `INSPECT_REDIRECTION_DOMAIN` and `TRUST_REDIRECTION_DOMAIN`. Default value is `INSPECT_REDIRECTION_DOMAIN`.
        pub firewall_domain_redirection_action: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The unique identifier of the firewall rule group where you want to create the rule.
        pub firewall_rule_group_id: pulumi_gestalt_rust::Output<String>,
        /// A name that lets you identify the rule, to manage and use it.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The setting that determines the processing order of the rule in the rule group. DNS Firewall processes the rules in a rule group by order of priority, starting from the lowest setting.
        pub priority: pulumi_gestalt_rust::Output<i32>,
        /// The query type you want the rule to evaluate. Additional details can be found [here](https://en.wikipedia.org/wiki/List_of_DNS_record_types)
        pub q_type: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResolverFirewallRuleArgs,
    ) -> ResolverFirewallRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let action_binding = args.action.get_output(context);
        let block_override_dns_type_binding = args
            .block_override_dns_type
            .get_output(context);
        let block_override_domain_binding = args
            .block_override_domain
            .get_output(context);
        let block_override_ttl_binding = args.block_override_ttl.get_output(context);
        let block_response_binding = args.block_response.get_output(context);
        let firewall_domain_list_id_binding = args
            .firewall_domain_list_id
            .get_output(context);
        let firewall_domain_redirection_action_binding = args
            .firewall_domain_redirection_action
            .get_output(context);
        let firewall_rule_group_id_binding = args
            .firewall_rule_group_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let q_type_binding = args.q_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:route53/resolverFirewallRule:ResolverFirewallRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "action".into(),
                    value: action_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blockOverrideDnsType".into(),
                    value: block_override_dns_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blockOverrideDomain".into(),
                    value: block_override_domain_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blockOverrideTtl".into(),
                    value: block_override_ttl_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blockResponse".into(),
                    value: block_response_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firewallDomainListId".into(),
                    value: firewall_domain_list_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firewallDomainRedirectionAction".into(),
                    value: firewall_domain_redirection_action_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firewallRuleGroupId".into(),
                    value: firewall_rule_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: priority_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "qType".into(),
                    value: q_type_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResolverFirewallRuleResult {
            action: o.get_field("action"),
            block_override_dns_type: o.get_field("blockOverrideDnsType"),
            block_override_domain: o.get_field("blockOverrideDomain"),
            block_override_ttl: o.get_field("blockOverrideTtl"),
            block_response: o.get_field("blockResponse"),
            firewall_domain_list_id: o.get_field("firewallDomainListId"),
            firewall_domain_redirection_action: o
                .get_field("firewallDomainRedirectionAction"),
            firewall_rule_group_id: o.get_field("firewallRuleGroupId"),
            name: o.get_field("name"),
            priority: o.get_field("priority"),
            q_type: o.get_field("qType"),
        }
    }
}
