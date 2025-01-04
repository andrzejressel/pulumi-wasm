pub mod get_resolver_forwarding_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResolverForwardingRuleArgs {
        /// ID of the Private DNS Resolver Forwarding Ruleset.
        #[builder(into)]
        pub dns_forwarding_ruleset_id: pulumi_wasm_rust::Output<String>,
        /// Name of the Private DNS Resolver Forwarding Rule.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetResolverForwardingRuleResult {
        pub dns_forwarding_ruleset_id: pulumi_wasm_rust::Output<String>,
        /// The domain name for the Private DNS Resolver Forwarding Rule.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// Is the Private DNS Resolver Forwarding Rule enabled?
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The metadata attached to the Private DNS Resolver Forwarding Rule.
        pub metadata: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of `target_dns_servers` block as defined below.
        pub target_dns_servers: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::privatedns::GetResolverForwardingRuleTargetDnsServer,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetResolverForwardingRuleArgs,
    ) -> GetResolverForwardingRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dns_forwarding_ruleset_id_binding = args
            .dns_forwarding_ruleset_id
            .get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:privatedns/getResolverForwardingRule:getResolverForwardingRule"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dnsForwardingRulesetId".into(),
                    value: &dns_forwarding_ruleset_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dnsForwardingRulesetId".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "metadata".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "targetDnsServers".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetResolverForwardingRuleResult {
            dns_forwarding_ruleset_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsForwardingRulesetId").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadata").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            target_dns_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetDnsServers").unwrap(),
            ),
        }
    }
}
