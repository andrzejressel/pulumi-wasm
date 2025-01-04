pub mod get_resolver_dns_forwarding_ruleset {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResolverDnsForwardingRulesetArgs {
        /// Name of the existing Private DNS Resolver Dns Forwarding Ruleset.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Name of the Resource Group where the Private DNS Resolver Dns Forwarding Ruleset exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetResolverDnsForwardingRulesetResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Private DNS Resolver Dns Forwarding Ruleset exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The IDs list of the Private DNS Resolver Outbound Endpoints that are linked to the Private DNS Resolver Dns Forwarding Ruleset.
        pub private_dns_resolver_outbound_endpoint_ids: pulumi_wasm_rust::Output<
            Vec<String>,
        >,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The tags assigned to the Private DNS Resolver Dns Forwarding Ruleset.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetResolverDnsForwardingRulesetArgs,
    ) -> GetResolverDnsForwardingRulesetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:privatedns/getResolverDnsForwardingRuleset:getResolverDnsForwardingRuleset"
                .into(),
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
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "privateDnsResolverOutboundEndpointIds".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetResolverDnsForwardingRulesetResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            private_dns_resolver_outbound_endpoint_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateDnsResolverOutboundEndpointIds").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
