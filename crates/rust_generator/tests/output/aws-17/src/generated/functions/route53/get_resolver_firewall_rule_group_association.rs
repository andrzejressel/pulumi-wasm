#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_resolver_firewall_rule_group_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResolverFirewallRuleGroupAssociationArgs {
        /// The identifier for the association.
        ///
        /// The following attribute is additionally exported:
        #[builder(into)]
        pub firewall_rule_group_association_id: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
    }
    #[allow(dead_code)]
    pub struct GetResolverFirewallRuleGroupAssociationResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub creation_time: pulumi_gestalt_rust::Output<String>,
        pub creator_request_id: pulumi_gestalt_rust::Output<String>,
        pub firewall_rule_group_association_id: pulumi_gestalt_rust::Output<String>,
        pub firewall_rule_group_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub managed_owner_name: pulumi_gestalt_rust::Output<String>,
        pub modification_time: pulumi_gestalt_rust::Output<String>,
        pub mutation_protection: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub priority: pulumi_gestalt_rust::Output<i32>,
        pub status: pulumi_gestalt_rust::Output<String>,
        pub status_message: pulumi_gestalt_rust::Output<String>,
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetResolverFirewallRuleGroupAssociationArgs,
    ) -> GetResolverFirewallRuleGroupAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let firewall_rule_group_association_id_binding = args
            .firewall_rule_group_association_id
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:route53/getResolverFirewallRuleGroupAssociation:getResolverFirewallRuleGroupAssociation"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firewallRuleGroupAssociationId".into(),
                    value: firewall_rule_group_association_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetResolverFirewallRuleGroupAssociationResult {
            arn: o.get_field("arn"),
            creation_time: o.get_field("creationTime"),
            creator_request_id: o.get_field("creatorRequestId"),
            firewall_rule_group_association_id: o
                .get_field("firewallRuleGroupAssociationId"),
            firewall_rule_group_id: o.get_field("firewallRuleGroupId"),
            id: o.get_field("id"),
            managed_owner_name: o.get_field("managedOwnerName"),
            modification_time: o.get_field("modificationTime"),
            mutation_protection: o.get_field("mutationProtection"),
            name: o.get_field("name"),
            priority: o.get_field("priority"),
            status: o.get_field("status"),
            status_message: o.get_field("statusMessage"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
