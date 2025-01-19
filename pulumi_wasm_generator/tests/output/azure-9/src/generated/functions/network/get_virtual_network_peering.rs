pub mod get_virtual_network_peering {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualNetworkPeeringArgs {
        /// The name of this virtual network peering.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the virtual network.
        #[builder(into)]
        pub virtual_network_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetVirtualNetworkPeeringResult {
        /// Controls if forwarded traffic from VMs in the remote virtual network is allowed.
        pub allow_forwarded_traffic: pulumi_wasm_rust::Output<bool>,
        /// Controls gatewayLinks can be used in the remote virtual network’s link to the local virtual network.
        pub allow_gateway_transit: pulumi_wasm_rust::Output<bool>,
        /// Controls if the traffic from the local virtual network can reach the remote virtual network.
        pub allow_virtual_network_access: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies whether only IPv6 address space is peered for Subnet peering.
        pub only_ipv6_peering_enabled: pulumi_wasm_rust::Output<bool>,
        /// Specifies whether complete Virtual Network address space is peered.
        pub peer_complete_virtual_networks_enabled: pulumi_wasm_rust::Output<bool>,
        /// The full Azure resource ID of the remote virtual network.
        pub remote_virtual_network_id: pulumi_wasm_rust::Output<String>,
        /// Controls if remote gateways can be used on the local virtual network.
        pub use_remote_gateways: pulumi_wasm_rust::Output<bool>,
        pub virtual_network_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetVirtualNetworkPeeringArgs) -> GetVirtualNetworkPeeringResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let virtual_network_id_binding = args.virtual_network_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getVirtualNetworkPeering:getVirtualNetworkPeering"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "virtualNetworkId".into(),
                    value: &virtual_network_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allowForwardedTraffic".into(),
                },
                register_interface::ResultField {
                    name: "allowGatewayTransit".into(),
                },
                register_interface::ResultField {
                    name: "allowVirtualNetworkAccess".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "onlyIpv6PeeringEnabled".into(),
                },
                register_interface::ResultField {
                    name: "peerCompleteVirtualNetworksEnabled".into(),
                },
                register_interface::ResultField {
                    name: "remoteVirtualNetworkId".into(),
                },
                register_interface::ResultField {
                    name: "useRemoteGateways".into(),
                },
                register_interface::ResultField {
                    name: "virtualNetworkId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetVirtualNetworkPeeringResult {
            allow_forwarded_traffic: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowForwardedTraffic").unwrap(),
            ),
            allow_gateway_transit: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowGatewayTransit").unwrap(),
            ),
            allow_virtual_network_access: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowVirtualNetworkAccess").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            only_ipv6_peering_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("onlyIpv6PeeringEnabled").unwrap(),
            ),
            peer_complete_virtual_networks_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerCompleteVirtualNetworksEnabled").unwrap(),
            ),
            remote_virtual_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("remoteVirtualNetworkId").unwrap(),
            ),
            use_remote_gateways: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("useRemoteGateways").unwrap(),
            ),
            virtual_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualNetworkId").unwrap(),
            ),
        }
    }
}
