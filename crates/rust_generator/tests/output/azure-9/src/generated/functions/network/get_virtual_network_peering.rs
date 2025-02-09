#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_virtual_network_peering {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualNetworkPeeringArgs {
        /// The name of this virtual network peering.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource ID of the virtual network.
        #[builder(into)]
        pub virtual_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVirtualNetworkPeeringResult {
        /// Controls if forwarded traffic from VMs in the remote virtual network is allowed.
        pub allow_forwarded_traffic: pulumi_gestalt_rust::Output<bool>,
        /// Controls gatewayLinks can be used in the remote virtual network’s link to the local virtual network.
        pub allow_gateway_transit: pulumi_gestalt_rust::Output<bool>,
        /// Controls if the traffic from the local virtual network can reach the remote virtual network.
        pub allow_virtual_network_access: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether only IPv6 address space is peered for Subnet peering.
        pub only_ipv6_peering_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Specifies whether complete Virtual Network address space is peered.
        pub peer_complete_virtual_networks_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The full Azure resource ID of the remote virtual network.
        pub remote_virtual_network_id: pulumi_gestalt_rust::Output<String>,
        /// Controls if remote gateways can be used on the local virtual network.
        pub use_remote_gateways: pulumi_gestalt_rust::Output<bool>,
        pub virtual_network_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetVirtualNetworkPeeringArgs,
    ) -> GetVirtualNetworkPeeringResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let virtual_network_id_binding = args.virtual_network_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:network/getVirtualNetworkPeering:getVirtualNetworkPeering"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualNetworkId".into(),
                    value: virtual_network_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVirtualNetworkPeeringResult {
            allow_forwarded_traffic: o.get_field("allowForwardedTraffic"),
            allow_gateway_transit: o.get_field("allowGatewayTransit"),
            allow_virtual_network_access: o.get_field("allowVirtualNetworkAccess"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            only_ipv6_peering_enabled: o.get_field("onlyIpv6PeeringEnabled"),
            peer_complete_virtual_networks_enabled: o
                .get_field("peerCompleteVirtualNetworksEnabled"),
            remote_virtual_network_id: o.get_field("remoteVirtualNetworkId"),
            use_remote_gateways: o.get_field("useRemoteGateways"),
            virtual_network_id: o.get_field("virtualNetworkId"),
        }
    }
}
