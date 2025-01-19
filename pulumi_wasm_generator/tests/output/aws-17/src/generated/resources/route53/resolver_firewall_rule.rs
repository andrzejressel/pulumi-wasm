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
pub mod resolver_firewall_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResolverFirewallRuleArgs {
        /// The action that DNS Firewall should take on a DNS query when it matches one of the domains in the rule's domain list. Valid values: `ALLOW`, `BLOCK`, `ALERT`.
        #[builder(into)]
        pub action: pulumi_wasm_rust::Output<String>,
        /// The DNS record's type. This determines the format of the record value that you provided in BlockOverrideDomain. Value values: `CNAME`.
        #[builder(into, default)]
        pub block_override_dns_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The custom DNS record to send back in response to the query.
        #[builder(into, default)]
        pub block_override_domain: pulumi_wasm_rust::Output<Option<String>>,
        /// The recommended amount of time, in seconds, for the DNS resolver or web browser to cache the provided override record. Minimum value of 0. Maximum value of 604800.
        #[builder(into, default)]
        pub block_override_ttl: pulumi_wasm_rust::Output<Option<i32>>,
        /// The way that you want DNS Firewall to block the request. Valid values: `NODATA`, `NXDOMAIN`, `OVERRIDE`.
        #[builder(into, default)]
        pub block_response: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the domain list that you want to use in the rule.
        #[builder(into)]
        pub firewall_domain_list_id: pulumi_wasm_rust::Output<String>,
        /// Evaluate DNS redirection in the DNS redirection chain, such as CNAME, DNAME, ot ALIAS. Valid values are `INSPECT_REDIRECTION_DOMAIN` and `TRUST_REDIRECTION_DOMAIN`. Default value is `INSPECT_REDIRECTION_DOMAIN`.
        #[builder(into, default)]
        pub firewall_domain_redirection_action: pulumi_wasm_rust::Output<Option<String>>,
        /// The unique identifier of the firewall rule group where you want to create the rule.
        #[builder(into)]
        pub firewall_rule_group_id: pulumi_wasm_rust::Output<String>,
        /// A name that lets you identify the rule, to manage and use it.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The setting that determines the processing order of the rule in the rule group. DNS Firewall processes the rules in a rule group by order of priority, starting from the lowest setting.
        #[builder(into)]
        pub priority: pulumi_wasm_rust::Output<i32>,
        /// The query type you want the rule to evaluate. Additional details can be found [here](https://en.wikipedia.org/wiki/List_of_DNS_record_types)
        #[builder(into, default)]
        pub q_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ResolverFirewallRuleResult {
        /// The action that DNS Firewall should take on a DNS query when it matches one of the domains in the rule's domain list. Valid values: `ALLOW`, `BLOCK`, `ALERT`.
        pub action: pulumi_wasm_rust::Output<String>,
        /// The DNS record's type. This determines the format of the record value that you provided in BlockOverrideDomain. Value values: `CNAME`.
        pub block_override_dns_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The custom DNS record to send back in response to the query.
        pub block_override_domain: pulumi_wasm_rust::Output<Option<String>>,
        /// The recommended amount of time, in seconds, for the DNS resolver or web browser to cache the provided override record. Minimum value of 0. Maximum value of 604800.
        pub block_override_ttl: pulumi_wasm_rust::Output<Option<i32>>,
        /// The way that you want DNS Firewall to block the request. Valid values: `NODATA`, `NXDOMAIN`, `OVERRIDE`.
        pub block_response: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the domain list that you want to use in the rule.
        pub firewall_domain_list_id: pulumi_wasm_rust::Output<String>,
        /// Evaluate DNS redirection in the DNS redirection chain, such as CNAME, DNAME, ot ALIAS. Valid values are `INSPECT_REDIRECTION_DOMAIN` and `TRUST_REDIRECTION_DOMAIN`. Default value is `INSPECT_REDIRECTION_DOMAIN`.
        pub firewall_domain_redirection_action: pulumi_wasm_rust::Output<Option<String>>,
        /// The unique identifier of the firewall rule group where you want to create the rule.
        pub firewall_rule_group_id: pulumi_wasm_rust::Output<String>,
        /// A name that lets you identify the rule, to manage and use it.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The setting that determines the processing order of the rule in the rule group. DNS Firewall processes the rules in a rule group by order of priority, starting from the lowest setting.
        pub priority: pulumi_wasm_rust::Output<i32>,
        /// The query type you want the rule to evaluate. Additional details can be found [here](https://en.wikipedia.org/wiki/List_of_DNS_record_types)
        pub q_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ResolverFirewallRuleArgs,
    ) -> ResolverFirewallRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_inner();
        let block_override_dns_type_binding = args.block_override_dns_type.get_inner();
        let block_override_domain_binding = args.block_override_domain.get_inner();
        let block_override_ttl_binding = args.block_override_ttl.get_inner();
        let block_response_binding = args.block_response.get_inner();
        let firewall_domain_list_id_binding = args.firewall_domain_list_id.get_inner();
        let firewall_domain_redirection_action_binding = args
            .firewall_domain_redirection_action
            .get_inner();
        let firewall_rule_group_id_binding = args.firewall_rule_group_id.get_inner();
        let name_binding = args.name.get_inner();
        let priority_binding = args.priority.get_inner();
        let q_type_binding = args.q_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53/resolverFirewallRule:ResolverFirewallRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "action".into(),
                    value: &action_binding,
                },
                register_interface::ObjectField {
                    name: "blockOverrideDnsType".into(),
                    value: &block_override_dns_type_binding,
                },
                register_interface::ObjectField {
                    name: "blockOverrideDomain".into(),
                    value: &block_override_domain_binding,
                },
                register_interface::ObjectField {
                    name: "blockOverrideTtl".into(),
                    value: &block_override_ttl_binding,
                },
                register_interface::ObjectField {
                    name: "blockResponse".into(),
                    value: &block_response_binding,
                },
                register_interface::ObjectField {
                    name: "firewallDomainListId".into(),
                    value: &firewall_domain_list_id_binding,
                },
                register_interface::ObjectField {
                    name: "firewallDomainRedirectionAction".into(),
                    value: &firewall_domain_redirection_action_binding,
                },
                register_interface::ObjectField {
                    name: "firewallRuleGroupId".into(),
                    value: &firewall_rule_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "qType".into(),
                    value: &q_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "action".into(),
                },
                register_interface::ResultField {
                    name: "blockOverrideDnsType".into(),
                },
                register_interface::ResultField {
                    name: "blockOverrideDomain".into(),
                },
                register_interface::ResultField {
                    name: "blockOverrideTtl".into(),
                },
                register_interface::ResultField {
                    name: "blockResponse".into(),
                },
                register_interface::ResultField {
                    name: "firewallDomainListId".into(),
                },
                register_interface::ResultField {
                    name: "firewallDomainRedirectionAction".into(),
                },
                register_interface::ResultField {
                    name: "firewallRuleGroupId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "priority".into(),
                },
                register_interface::ResultField {
                    name: "qType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ResolverFirewallRuleResult {
            action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("action").unwrap(),
            ),
            block_override_dns_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blockOverrideDnsType").unwrap(),
            ),
            block_override_domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blockOverrideDomain").unwrap(),
            ),
            block_override_ttl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blockOverrideTtl").unwrap(),
            ),
            block_response: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blockResponse").unwrap(),
            ),
            firewall_domain_list_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firewallDomainListId").unwrap(),
            ),
            firewall_domain_redirection_action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firewallDomainRedirectionAction").unwrap(),
            ),
            firewall_rule_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firewallRuleGroupId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priority").unwrap(),
            ),
            q_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("qType").unwrap(),
            ),
        }
    }
}
