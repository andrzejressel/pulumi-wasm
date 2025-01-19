pub mod get_gateway_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGatewayConnectionArgs {
        /// Specifies the name of the Virtual Network Gateway Connection.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the resource group the Virtual Network Gateway Connection is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetGatewayConnectionResult {
        /// The authorization key associated with the
        /// Express Route Circuit. This field is present only if the type is an
        /// ExpressRoute connection.
        pub authorization_key: pulumi_wasm_rust::Output<String>,
        pub connection_protocol: pulumi_wasm_rust::Output<String>,
        /// The dead peer detection timeout of this connection in seconds.
        pub dpd_timeout_seconds: pulumi_wasm_rust::Output<i32>,
        pub egress_bytes_transferred: pulumi_wasm_rust::Output<i32>,
        /// If `true`, BGP (Border Gateway Protocol) is enabled
        /// for this connection.
        pub enable_bgp: pulumi_wasm_rust::Output<bool>,
        /// The ID of the Express Route Circuit
        /// (i.e. when `type` is `ExpressRoute`).
        pub express_route_circuit_id: pulumi_wasm_rust::Output<String>,
        /// If `true`, data packets will bypass ExpressRoute Gateway for data forwarding. This is only valid for ExpressRoute connections.
        pub express_route_gateway_bypass: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub ingress_bytes_transferred: pulumi_wasm_rust::Output<i32>,
        /// (Optional) A `ipsec_policy` block which is documented below.
        /// Only a single policy can be defined for a connection. For details on
        /// custom policies refer to [the relevant section in the Azure documentation](https://docs.microsoft.com/azure/vpn-gateway/vpn-gateway-ipsecikepolicy-rm-powershell).
        pub ipsec_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetGatewayConnectionIpsecPolicy>,
        >,
        /// Use private local Azure IP for the connection.
        pub local_azure_ip_address_enabled: pulumi_wasm_rust::Output<bool>,
        /// The ID of the local network gateway
        /// when a Site-to-Site connection (i.e. when `type` is `IPsec`).
        pub local_network_gateway_id: pulumi_wasm_rust::Output<String>,
        /// The location/region where the connection is
        /// located.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the peer virtual
        /// network gateway when a VNet-to-VNet connection (i.e. when `type`
        /// is `Vnet2Vnet`).
        pub peer_virtual_network_gateway_id: pulumi_wasm_rust::Output<String>,
        /// If `true`, data packets will bypass the Express Route gateway when accessing private-links.
        /// This is only valid for ExpressRoute connections, on the conditions described in [the relevant section in the Azure documentation](https://learn.microsoft.com/en-us/azure/expressroute/expressroute-howto-linkvnet-arm#fastpath-virtual-network-peering-user-defined-routes-udrs-and-private-link-support-for-expressroute-direct-connections)
        pub private_link_fast_path_enabled: pulumi_wasm_rust::Output<bool>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        pub resource_guid: pulumi_wasm_rust::Output<String>,
        /// The routing weight.
        pub routing_weight: pulumi_wasm_rust::Output<i32>,
        /// The shared IPSec key.
        pub shared_key: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// One or more `traffic_selector_policy` blocks which are documented below.
        /// A `traffic_selector_policy` allows to specify a traffic selector policy proposal to be used in a virtual network gateway connection.
        /// For details about traffic selectors refer to [the relevant section in the Azure documentation](https://docs.microsoft.com/azure/vpn-gateway/vpn-gateway-connect-multiple-policybased-rm-ps).
        pub traffic_selector_policies: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetGatewayConnectionTrafficSelectorPolicy,
            >,
        >,
        /// The type of connection. Valid options are `IPsec`
        /// (Site-to-Site), `ExpressRoute` (ExpressRoute), and `Vnet2Vnet` (VNet-to-VNet).
        pub type_: pulumi_wasm_rust::Output<String>,
        /// If `true`, policy-based traffic
        /// selectors are enabled for this connection. Enabling policy-based traffic
        /// selectors requires an `ipsec_policy` block.
        pub use_policy_based_traffic_selectors: pulumi_wasm_rust::Output<bool>,
        /// The ID of the Virtual Network Gateway
        /// in which the connection is created.
        pub virtual_network_gateway_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetGatewayConnectionArgs) -> GetGatewayConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "authorizationKey".into(),
                },
                register_interface::ResultField {
                    name: "connectionProtocol".into(),
                },
                register_interface::ResultField {
                    name: "dpdTimeoutSeconds".into(),
                },
                register_interface::ResultField {
                    name: "egressBytesTransferred".into(),
                },
                register_interface::ResultField {
                    name: "enableBgp".into(),
                },
                register_interface::ResultField {
                    name: "expressRouteCircuitId".into(),
                },
                register_interface::ResultField {
                    name: "expressRouteGatewayBypass".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ingressBytesTransferred".into(),
                },
                register_interface::ResultField {
                    name: "ipsecPolicies".into(),
                },
                register_interface::ResultField {
                    name: "localAzureIpAddressEnabled".into(),
                },
                register_interface::ResultField {
                    name: "localNetworkGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "peerVirtualNetworkGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "privateLinkFastPathEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGuid".into(),
                },
                register_interface::ResultField {
                    name: "routingWeight".into(),
                },
                register_interface::ResultField {
                    name: "sharedKey".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "trafficSelectorPolicies".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "usePolicyBasedTrafficSelectors".into(),
                },
                register_interface::ResultField {
                    name: "virtualNetworkGatewayId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetGatewayConnectionResult {
            authorization_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizationKey").unwrap(),
            ),
            connection_protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionProtocol").unwrap(),
            ),
            dpd_timeout_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dpdTimeoutSeconds").unwrap(),
            ),
            egress_bytes_transferred: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("egressBytesTransferred").unwrap(),
            ),
            enable_bgp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableBgp").unwrap(),
            ),
            express_route_circuit_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expressRouteCircuitId").unwrap(),
            ),
            express_route_gateway_bypass: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expressRouteGatewayBypass").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ingress_bytes_transferred: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ingressBytesTransferred").unwrap(),
            ),
            ipsec_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipsecPolicies").unwrap(),
            ),
            local_azure_ip_address_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localAzureIpAddressEnabled").unwrap(),
            ),
            local_network_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localNetworkGatewayId").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            peer_virtual_network_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerVirtualNetworkGatewayId").unwrap(),
            ),
            private_link_fast_path_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateLinkFastPathEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            resource_guid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGuid").unwrap(),
            ),
            routing_weight: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routingWeight").unwrap(),
            ),
            shared_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sharedKey").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            traffic_selector_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trafficSelectorPolicies").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            use_policy_based_traffic_selectors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("usePolicyBasedTrafficSelectors").unwrap(),
            ),
            virtual_network_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualNetworkGatewayId").unwrap(),
            ),
        }
    }
}
