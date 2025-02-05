pub mod get_resolver_firewall_rule_group_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResolverFirewallRuleGroupAssociationArgs {
        /// The identifier for the association.
        ///
        /// The following attribute is additionally exported:
        #[builder(into)]
        pub firewall_rule_group_association_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetResolverFirewallRuleGroupAssociationResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        pub creation_time: pulumi_wasm_rust::Output<String>,
        pub creator_request_id: pulumi_wasm_rust::Output<String>,
        pub firewall_rule_group_association_id: pulumi_wasm_rust::Output<String>,
        pub firewall_rule_group_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub managed_owner_name: pulumi_wasm_rust::Output<String>,
        pub modification_time: pulumi_wasm_rust::Output<String>,
        pub mutation_protection: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub priority: pulumi_wasm_rust::Output<i32>,
        pub status: pulumi_wasm_rust::Output<String>,
        pub status_message: pulumi_wasm_rust::Output<String>,
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetResolverFirewallRuleGroupAssociationArgs,
    ) -> GetResolverFirewallRuleGroupAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let firewall_rule_group_association_id_binding = args
            .firewall_rule_group_association_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:route53/getResolverFirewallRuleGroupAssociation:getResolverFirewallRuleGroupAssociation"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "firewallRuleGroupAssociationId".into(),
                    value: &firewall_rule_group_association_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetResolverFirewallRuleGroupAssociationResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            creation_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("creationTime"),
            ),
            creator_request_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("creatorRequestId"),
            ),
            firewall_rule_group_association_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("firewallRuleGroupAssociationId"),
            ),
            firewall_rule_group_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("firewallRuleGroupId"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            managed_owner_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("managedOwnerName"),
            ),
            modification_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("modificationTime"),
            ),
            mutation_protection: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("mutationProtection"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            priority: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("priority"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            status_message: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("statusMessage"),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
