pub mod get_resolver_firewall_rules {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResolverFirewallRulesArgs {
        /// The action that DNS Firewall should take on a DNS query when it matches one of the domains in the rule's domain list.
        #[builder(into, default)]
        pub action: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The unique identifier of the firewall rule group that you want to retrieve the rules for.
        #[builder(into)]
        pub firewall_rule_group_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The setting that determines the processing order of the rules in a rule group.
        #[builder(into, default)]
        pub priority: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetResolverFirewallRulesArgs,
    ) -> GetResolverFirewallRulesResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_output(context).get_inner();
        let firewall_rule_group_id_binding = args
            .firewall_rule_group_id
            .get_output(context)
            .get_inner();
        let priority_binding = args.priority.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:route53/getResolverFirewallRules:getResolverFirewallRules"
                .into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetResolverFirewallRulesResult {
            action: pulumi_wasm_rust::__private::into_domain(o.extract_field("action")),
            firewall_rule_group_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("firewallRuleGroupId"),
            ),
            firewall_rules: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("firewallRules"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            priority: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("priority"),
            ),
        }
    }
}
