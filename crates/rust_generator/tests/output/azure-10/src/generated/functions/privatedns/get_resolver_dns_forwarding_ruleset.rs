pub mod get_resolver_dns_forwarding_ruleset {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResolverDnsForwardingRulesetArgs {
        /// Name of the existing Private DNS Resolver Dns Forwarding Ruleset.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the Resource Group where the Private DNS Resolver Dns Forwarding Ruleset exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
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
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetResolverDnsForwardingRulesetArgs,
    ) -> GetResolverDnsForwardingRulesetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:privatedns/getResolverDnsForwardingRuleset:getResolverDnsForwardingRuleset"
                .into(),
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
        GetResolverDnsForwardingRulesetResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            private_dns_resolver_outbound_endpoint_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateDnsResolverOutboundEndpointIds"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
