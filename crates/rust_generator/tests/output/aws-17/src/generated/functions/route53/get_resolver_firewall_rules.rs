#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_resolver_firewall_rules {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResolverFirewallRulesArgs {
        /// The action that DNS Firewall should take on a DNS query when it matches one of the domains in the rule's domain list.
        #[builder(into, default)]
        pub action: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The unique identifier of the firewall rule group that you want to retrieve the rules for.
        #[builder(into)]
        pub firewall_rule_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The setting that determines the processing order of the rules in a rule group.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct GetResolverFirewallRulesResult {
        pub action: pulumi_gestalt_rust::Output<Option<String>>,
        pub firewall_rule_group_id: pulumi_gestalt_rust::Output<String>,
        /// List with information about the firewall rules. See details below.
        pub firewall_rules: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::route53::GetResolverFirewallRulesFirewallRule,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub priority: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetResolverFirewallRulesArgs,
    ) -> GetResolverFirewallRulesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let action_binding = args.action.get_output(context);
        let firewall_rule_group_id_binding = args
            .firewall_rule_group_id
            .get_output(context);
        let priority_binding = args.priority.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:route53/getResolverFirewallRules:getResolverFirewallRules"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "action".into(),
                    value: &action_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firewallRuleGroupId".into(),
                    value: &firewall_rule_group_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: &priority_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetResolverFirewallRulesResult {
            action: o.get_field("action"),
            firewall_rule_group_id: o.get_field("firewallRuleGroupId"),
            firewall_rules: o.get_field("firewallRules"),
            id: o.get_field("id"),
            priority: o.get_field("priority"),
        }
    }
}
