pub mod get_firewall_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFirewallPolicyArgs {
        /// The name of this Firewall Policy.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Firewall Policy exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFirewallPolicyResult {
        pub base_policy_id: pulumi_wasm_rust::Output<String>,
        pub child_policies: pulumi_wasm_rust::Output<Vec<String>>,
        pub dns: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetFirewallPolicyDn>,
        >,
        pub firewalls: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        pub rule_collection_groups: pulumi_wasm_rust::Output<Vec<String>>,
        /// A mapping of tags assigned to the Firewall Policy.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub threat_intelligence_allowlists: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetFirewallPolicyThreatIntelligenceAllowlist,
            >,
        >,
        pub threat_intelligence_mode: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetFirewallPolicyArgs,
    ) -> GetFirewallPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "basePolicyId".into(),
                },
                register_interface::ResultField {
                    name: "childPolicies".into(),
                },
                register_interface::ResultField {
                    name: "dns".into(),
                },
                register_interface::ResultField {
                    name: "firewalls".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "ruleCollectionGroups".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "threatIntelligenceAllowlists".into(),
                },
                register_interface::ResultField {
                    name: "threatIntelligenceMode".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetFirewallPolicyResult {
            base_policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("basePolicyId").unwrap(),
            ),
            child_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("childPolicies").unwrap(),
            ),
            dns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dns").unwrap(),
            ),
            firewalls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firewalls").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            rule_collection_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ruleCollectionGroups").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            threat_intelligence_allowlists: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("threatIntelligenceAllowlists").unwrap(),
            ),
            threat_intelligence_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("threatIntelligenceMode").unwrap(),
            ),
        }
    }
}
