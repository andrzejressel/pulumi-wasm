#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_resolver_firewall_rule_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResolverFirewallRuleGroupArgs {
        /// The ID of the rule group.
        ///
        /// The following attribute is additionally exported:
        #[builder(into)]
        pub firewall_rule_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetResolverFirewallRuleGroupResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub creation_time: pulumi_gestalt_rust::Output<String>,
        pub creator_request_id: pulumi_gestalt_rust::Output<String>,
        pub firewall_rule_group_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub modification_time: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        pub rule_count: pulumi_gestalt_rust::Output<i32>,
        pub share_status: pulumi_gestalt_rust::Output<String>,
        pub status: pulumi_gestalt_rust::Output<String>,
        pub status_message: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetResolverFirewallRuleGroupArgs,
    ) -> GetResolverFirewallRuleGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let firewall_rule_group_id_binding = args
            .firewall_rule_group_id
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:route53/getResolverFirewallRuleGroup:getResolverFirewallRuleGroup"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firewallRuleGroupId".into(),
                    value: &firewall_rule_group_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetResolverFirewallRuleGroupResult {
            arn: o.get_field("arn"),
            creation_time: o.get_field("creationTime"),
            creator_request_id: o.get_field("creatorRequestId"),
            firewall_rule_group_id: o.get_field("firewallRuleGroupId"),
            id: o.get_field("id"),
            modification_time: o.get_field("modificationTime"),
            name: o.get_field("name"),
            owner_id: o.get_field("ownerId"),
            rule_count: o.get_field("ruleCount"),
            share_status: o.get_field("shareStatus"),
            status: o.get_field("status"),
            status_message: o.get_field("statusMessage"),
        }
    }
}
