#[allow(clippy::doc_lazy_continuation)]
pub mod get_resolver_inbound_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResolverInboundEndpointArgs {
        /// Name of the Private DNS Resolver Inbound Endpoint.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the Private DNS Resolver.
        #[builder(into)]
        pub private_dns_resolver_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetResolverInboundEndpointResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A list of `ip_configurations` block as defined below.
        pub ip_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::privatedns::GetResolverInboundEndpointIpConfiguration,
            >,
        >,
        /// The Azure Region where the Private DNS Resolver Inbound Endpoint exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub private_dns_resolver_id: pulumi_gestalt_rust::Output<String>,
        /// The tags assigned to the Private DNS Resolver Inbound Endpoint.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetResolverInboundEndpointArgs,
    ) -> GetResolverInboundEndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetResolverInboundEndpointResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ip_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipConfigurations"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            private_dns_resolver_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateDnsResolverId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
