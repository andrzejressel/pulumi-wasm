#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_resolver_outbound_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResolverOutboundEndpointArgs {
        /// Name of the Private DNS Resolver Outbound Endpoint.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the Private DNS Resolver Outbound Endpoint.
        #[builder(into)]
        pub private_dns_resolver_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetResolverOutboundEndpointResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Private DNS Resolver Outbound Endpoint exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub private_dns_resolver_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Subnet that is linked to the Private DNS Resolver Outbound Endpoint.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        /// The tags assigned to the Private DNS Resolver Outbound Endpoint.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetResolverOutboundEndpointArgs,
    ) -> GetResolverOutboundEndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let private_dns_resolver_id_binding = args
            .private_dns_resolver_id
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:privatedns/getResolverOutboundEndpoint:getResolverOutboundEndpoint"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateDnsResolverId".into(),
                    value: private_dns_resolver_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetResolverOutboundEndpointResult {
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            private_dns_resolver_id: o.get_field("privateDnsResolverId"),
            subnet_id: o.get_field("subnetId"),
            tags: o.get_field("tags"),
        }
    }
}
