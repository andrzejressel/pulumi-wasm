pub mod get_gateway_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGatewayConnectionArgs {
        /// Specifies the name of the Virtual Network Gateway Connection.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group the Virtual Network Gateway Connection is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetGatewayConnectionResult {
        /// The authorization key associated with the
        /// Express Route Circuit. This field is present only if the type is an
        /// ExpressRoute connection.
        pub authorization_key: pulumi_gestalt_rust::Output<String>,
        pub connection_protocol: pulumi_gestalt_rust::Output<String>,
        /// The dead peer detection timeout of this connection in seconds.
        pub dpd_timeout_seconds: pulumi_gestalt_rust::Output<i32>,
        pub egress_bytes_transferred: pulumi_gestalt_rust::Output<i32>,
        /// If `true`, BGP (Border Gateway Protocol) is enabled
        /// for this connection.
        pub enable_bgp: pulumi_gestalt_rust::Output<bool>,
        /// The ID of the Express Route Circuit
        /// (i.e. when `type` is `ExpressRoute`).
        pub express_route_circuit_id: pulumi_gestalt_rust::Output<String>,
        /// If `true`, data packets will bypass ExpressRoute Gateway for data forwarding. This is only valid for ExpressRoute connections.
        pub express_route_gateway_bypass: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub ingress_bytes_transferred: pulumi_gestalt_rust::Output<i32>,
        /// (Optional) A `ipsec_policy` block which is documented below.
        /// Only a single policy can be defined for a connection. For details on
        /// custom policies refer to [the relevant section in the Azure documentation](https://docs.microsoft.com/azure/vpn-gateway/vpn-gateway-ipsecikepolicy-rm-powershell).
        pub ipsec_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetGatewayConnectionIpsecPolicy>,
        >,
        /// Use private local Azure IP for the connection.
        pub local_azure_ip_address_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The ID of the local network gateway
        /// when a Site-to-Site connection (i.e. when `type` is `IPsec`).
        pub local_network_gateway_id: pulumi_gestalt_rust::Output<String>,
        /// The location/region where the connection is
        /// located.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the peer virtual
        /// network gateway when a VNet-to-VNet connection (i.e. when `type`
        /// is `Vnet2Vnet`).
        pub peer_virtual_network_gateway_id: pulumi_gestalt_rust::Output<String>,
        /// If `true`, data packets will bypass the Express Route gateway when accessing private-links.
        /// This is only valid for ExpressRoute connections, on the conditions described in [the relevant section in the Azure documentation](https://learn.microsoft.com/en-us/azure/expressroute/expressroute-howto-linkvnet-arm#fastpath-virtual-network-peering-user-defined-routes-udrs-and-private-link-support-for-expressroute-direct-connections)
        pub private_link_fast_path_enabled: pulumi_gestalt_rust::Output<bool>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub resource_guid: pulumi_gestalt_rust::Output<String>,
        /// The routing weight.
        pub routing_weight: pulumi_gestalt_rust::Output<i32>,
        /// The shared IPSec key.
        pub shared_key: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// One or more `traffic_selector_policy` blocks which are documented below.
        /// A `traffic_selector_policy` allows to specify a traffic selector policy proposal to be used in a virtual network gateway connection.
        /// For details about traffic selectors refer to [the relevant section in the Azure documentation](https://docs.microsoft.com/azure/vpn-gateway/vpn-gateway-connect-multiple-policybased-rm-ps).
        pub traffic_selector_policies: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetGatewayConnectionTrafficSelectorPolicy,
            >,
        >,
        /// The type of connection. Valid options are `IPsec`
        /// (Site-to-Site), `ExpressRoute` (ExpressRoute), and `Vnet2Vnet` (VNet-to-VNet).
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// If `true`, policy-based traffic
        /// selectors are enabled for this connection. Enabling policy-based traffic
        /// selectors requires an `ipsec_policy` block.
        pub use_policy_based_traffic_selectors: pulumi_gestalt_rust::Output<bool>,
        /// The ID of the Virtual Network Gateway
        /// in which the connection is created.
        pub virtual_network_gateway_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetGatewayConnectionArgs,
    ) -> GetGatewayConnectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getGatewayConnection:getGatewayConnection".into(),
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
        GetGatewayConnectionResult {
            authorization_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authorizationKey"),
            ),
            connection_protocol: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("connectionProtocol"),
            ),
            dpd_timeout_seconds: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dpdTimeoutSeconds"),
            ),
            egress_bytes_transferred: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("egressBytesTransferred"),
            ),
            enable_bgp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableBgp"),
            ),
            express_route_circuit_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expressRouteCircuitId"),
            ),
            express_route_gateway_bypass: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expressRouteGatewayBypass"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ingress_bytes_transferred: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ingressBytesTransferred"),
            ),
            ipsec_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipsecPolicies"),
            ),
            local_azure_ip_address_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("localAzureIpAddressEnabled"),
            ),
            local_network_gateway_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("localNetworkGatewayId"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            peer_virtual_network_gateway_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("peerVirtualNetworkGatewayId"),
            ),
            private_link_fast_path_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateLinkFastPathEnabled"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            resource_guid: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGuid"),
            ),
            routing_weight: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("routingWeight"),
            ),
            shared_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sharedKey"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            traffic_selector_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("trafficSelectorPolicies"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            use_policy_based_traffic_selectors: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("usePolicyBasedTrafficSelectors"),
            ),
            virtual_network_gateway_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("virtualNetworkGatewayId"),
            ),
        }
    }
}
