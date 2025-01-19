pub mod get_resolver_firewall_rule_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResolverFirewallRuleGroupArgs {
        /// The ID of the rule group.
        ///
        /// The following attribute is additionally exported:
        #[builder(into)]
        pub firewall_rule_group_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetResolverFirewallRuleGroupResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        pub creation_time: pulumi_wasm_rust::Output<String>,
        pub creator_request_id: pulumi_wasm_rust::Output<String>,
        pub firewall_rule_group_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub modification_time: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub owner_id: pulumi_wasm_rust::Output<String>,
        pub rule_count: pulumi_wasm_rust::Output<i32>,
        pub share_status: pulumi_wasm_rust::Output<String>,
        pub status: pulumi_wasm_rust::Output<String>,
        pub status_message: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetResolverFirewallRuleGroupArgs,
    ) -> GetResolverFirewallRuleGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let firewall_rule_group_id_binding = args.firewall_rule_group_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:route53/getResolverFirewallRuleGroup:getResolverFirewallRuleGroup"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "firewallRuleGroupId".into(),
                    value: &firewall_rule_group_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "creationTime".into(),
                },
                register_interface::ResultField {
                    name: "creatorRequestId".into(),
                },
                register_interface::ResultField {
                    name: "firewallRuleGroupId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "modificationTime".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "ruleCount".into(),
                },
                register_interface::ResultField {
                    name: "shareStatus".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "statusMessage".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetResolverFirewallRuleGroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            creation_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTime").unwrap(),
            ),
            creator_request_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creatorRequestId").unwrap(),
            ),
            firewall_rule_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firewallRuleGroupId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            modification_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modificationTime").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            rule_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ruleCount").unwrap(),
            ),
            share_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shareStatus").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            status_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statusMessage").unwrap(),
            ),
        }
    }
}
