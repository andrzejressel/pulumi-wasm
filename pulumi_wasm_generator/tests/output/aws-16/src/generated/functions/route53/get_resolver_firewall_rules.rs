pub mod get_resolver_firewall_rules {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResolverFirewallRulesArgs {
        /// The action that DNS Firewall should take on a DNS query when it matches one of the domains in the rule's domain list.
        #[builder(into, default)]
        pub action: pulumi_wasm_rust::Output<Option<String>>,
        /// The unique identifier of the firewall rule group that you want to retrieve the rules for.
        #[builder(into)]
        pub firewall_rule_group_id: pulumi_wasm_rust::Output<String>,
        /// The setting that determines the processing order of the rules in a rule group.
        #[builder(into, default)]
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct GetResolverFirewallRulesResult {
        pub action: pulumi_wasm_rust::Output<Option<String>>,
        pub firewall_rule_group_id: pulumi_wasm_rust::Output<String>,
        /// List with information about the firewall rules. See details below.
        pub firewall_rules: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::route53::GetResolverFirewallRulesFirewallRule,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetResolverFirewallRulesArgs) -> GetResolverFirewallRulesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_inner();
        let firewall_rule_group_id_binding = args.firewall_rule_group_id.get_inner();
        let priority_binding = args.priority.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:route53/getResolverFirewallRules:getResolverFirewallRules"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "action".into(),
                    value: &action_binding,
                },
                register_interface::ObjectField {
                    name: "firewallRuleGroupId".into(),
                    value: &firewall_rule_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "action".into(),
                },
                register_interface::ResultField {
                    name: "firewallRuleGroupId".into(),
                },
                register_interface::ResultField {
                    name: "firewallRules".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "priority".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetResolverFirewallRulesResult {
            action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("action").unwrap(),
            ),
            firewall_rule_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firewallRuleGroupId").unwrap(),
            ),
            firewall_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firewallRules").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priority").unwrap(),
            ),
        }
    }
}
