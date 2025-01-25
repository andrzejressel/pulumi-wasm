pub mod get_resolver_inbound_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResolverInboundEndpointArgs {
        /// Name of the Private DNS Resolver Inbound Endpoint.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// ID of the Private DNS Resolver.
        #[builder(into)]
        pub private_dns_resolver_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetResolverInboundEndpointResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A list of `ip_configurations` block as defined below.
        pub ip_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::privatedns::GetResolverInboundEndpointIpConfiguration,
            >,
        >,
        /// The Azure Region where the Private DNS Resolver Inbound Endpoint exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub private_dns_resolver_id: pulumi_wasm_rust::Output<String>,
        /// The tags assigned to the Private DNS Resolver Inbound Endpoint.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetResolverInboundEndpointArgs,
    ) -> GetResolverInboundEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let private_dns_resolver_id_binding = args
            .private_dns_resolver_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:privatedns/getResolverInboundEndpoint:getResolverInboundEndpoint"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "privateDnsResolverId".into(),
                    value: &private_dns_resolver_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "privateDnsResolverId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetResolverInboundEndpointResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ip_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipConfigurations").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            private_dns_resolver_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateDnsResolverId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
