#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_firewall_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFirewallPolicyArgs {
        /// The name of this Firewall Policy.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Firewall Policy exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFirewallPolicyResult {
        pub base_policy_id: pulumi_gestalt_rust::Output<String>,
        pub child_policies: pulumi_gestalt_rust::Output<Vec<String>>,
        pub dns: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetFirewallPolicyDn>,
        >,
        pub firewalls: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub rule_collection_groups: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A mapping of tags assigned to the Firewall Policy.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub threat_intelligence_allowlists: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetFirewallPolicyThreatIntelligenceAllowlist,
            >,
        >,
        pub threat_intelligence_mode: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetFirewallPolicyArgs,
    ) -> GetFirewallPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getFirewallPolicy:getFirewallPolicy".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetFirewallPolicyResult {
            base_policy_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("basePolicyId"),
            ),
            child_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("childPolicies"),
            ),
            dns: pulumi_gestalt_rust::__private::into_domain(o.extract_field("dns")),
            firewalls: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("firewalls"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            rule_collection_groups: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ruleCollectionGroups"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            threat_intelligence_allowlists: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("threatIntelligenceAllowlists"),
            ),
            threat_intelligence_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("threatIntelligenceMode"),
            ),
        }
    }
}
