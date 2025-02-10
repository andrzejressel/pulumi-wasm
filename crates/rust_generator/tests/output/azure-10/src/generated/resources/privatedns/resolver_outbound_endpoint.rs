/// Manages a Private DNS Resolver Outbound Endpoint.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: west europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       addressSpaces:
///         - 10.0.0.0/16
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: outbounddns
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.0.64/28
///       delegations:
///         - name: Microsoft.Network.dnsResolvers
///           serviceDelegation:
///             actions:
///               - Microsoft.Network/virtualNetworks/subnets/join/action
///             name: Microsoft.Network/dnsResolvers
///   exampleResolver:
///     type: azure:privatedns:Resolver
///     name: example
///     properties:
///       name: example-resolver
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       virtualNetworkId: ${exampleVirtualNetwork.id}
///   exampleResolverOutboundEndpoint:
///     type: azure:privatedns:ResolverOutboundEndpoint
///     name: example
///     properties:
///       name: example-endpoint
///       privateDnsResolverId: ${exampleResolver.id}
///       location: ${exampleResolver.location}
///       subnetId: ${exampleSubnet.id}
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// Private DNS Resolver Outbound Endpoint can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:privatedns/resolverOutboundEndpoint:ResolverOutboundEndpoint example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Network/dnsResolvers/dnsResolver1/outboundEndpoints/outboundEndpoint1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resolver_outbound_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResolverOutboundEndpointArgs {
        /// Specifies the Azure Region where the Private DNS Resolver Outbound Endpoint should exist. Changing this forces a new Private DNS Resolver Outbound Endpoint to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name which should be used for this Private DNS Resolver Outbound Endpoint. Changing this forces a new Private DNS Resolver Outbound Endpoint to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of the Private DNS Resolver Outbound Endpoint. Changing this forces a new Private DNS Resolver Outbound Endpoint to be created.
        #[builder(into)]
        pub private_dns_resolver_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Subnet that is linked to the Private DNS Resolver Outbound Endpoint. Changing this forces a new resource to be created.
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Private DNS Resolver Outbound Endpoint.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ResolverOutboundEndpointResult {
        /// Specifies the Azure Region where the Private DNS Resolver Outbound Endpoint should exist. Changing this forces a new Private DNS Resolver Outbound Endpoint to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this Private DNS Resolver Outbound Endpoint. Changing this forces a new Private DNS Resolver Outbound Endpoint to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the Private DNS Resolver Outbound Endpoint. Changing this forces a new Private DNS Resolver Outbound Endpoint to be created.
        pub private_dns_resolver_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Subnet that is linked to the Private DNS Resolver Outbound Endpoint. Changing this forces a new resource to be created.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Private DNS Resolver Outbound Endpoint.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResolverOutboundEndpointArgs,
    ) -> ResolverOutboundEndpointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let private_dns_resolver_id_binding = args
            .private_dns_resolver_id
            .get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:privatedns/resolverOutboundEndpoint:ResolverOutboundEndpoint"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateDnsResolverId".into(),
                    value: private_dns_resolver_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: subnet_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResolverOutboundEndpointResult {
            location: o.get_field("location"),
            name: o.get_field("name"),
            private_dns_resolver_id: o.get_field("privateDnsResolverId"),
            subnet_id: o.get_field("subnetId"),
            tags: o.get_field("tags"),
        }
    }
}
