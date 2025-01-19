pub mod get_virtual_network_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualNetworkGatewayArgs {
        /// Specifies the name of the Virtual Network Gateway.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the resource group the Virtual Network Gateway is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetVirtualNetworkGatewayResult {
        /// Is this an Active-Active Gateway?
        pub active_active: pulumi_wasm_rust::Output<bool>,
        pub bgp_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetVirtualNetworkGatewayBgpSetting>,
        >,
        pub custom_routes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetVirtualNetworkGatewayCustomRoute>,
        >,
        /// The ID of the local network gateway
        /// through which outbound Internet traffic from the virtual network in which the
        /// gateway is created will be routed (*forced tunneling*). Refer to the
        /// [Azure documentation on forced tunneling](https://docs.microsoft.com/azure/vpn-gateway/vpn-gateway-forced-tunneling-rm).
        pub default_local_network_gateway_id: pulumi_wasm_rust::Output<String>,
        /// Will BGP (Border Gateway Protocol) will be enabled
        /// for this Virtual Network Gateway.
        pub enable_bgp: pulumi_wasm_rust::Output<bool>,
        /// The Generation of the Virtual Network Gateway.
        pub generation: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// One or two `ip_configuration` blocks documented below.
        pub ip_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetVirtualNetworkGatewayIpConfiguration,
            >,
        >,
        /// The location/region where the Virtual Network Gateway is located.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The user-defined name of the root certificate.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether a private IP will be used for this  gateway for connections.
        pub private_ip_address_enabled: pulumi_wasm_rust::Output<bool>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Configuration of the size and capacity of the Virtual Network Gateway.
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The type of the Virtual Network Gateway.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// A `vpn_client_configuration` block which is documented below.
        pub vpn_client_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetVirtualNetworkGatewayVpnClientConfiguration,
            >,
        >,
        /// The routing type of the Virtual Network Gateway.
        pub vpn_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetVirtualNetworkGatewayArgs) -> GetVirtualNetworkGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getVirtualNetworkGateway:getVirtualNetworkGateway"
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "activeActive".into(),
                },
                register_interface::ResultField {
                    name: "bgpSettings".into(),
                },
                register_interface::ResultField {
                    name: "customRoutes".into(),
                },
                register_interface::ResultField {
                    name: "defaultLocalNetworkGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "enableBgp".into(),
                },
                register_interface::ResultField {
                    name: "generation".into(),
                },
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
                    name: "privateIpAddressEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "vpnClientConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "vpnType".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetVirtualNetworkGatewayResult {
            active_active: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("activeActive").unwrap(),
            ),
            bgp_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bgpSettings").unwrap(),
            ),
            custom_routes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customRoutes").unwrap(),
            ),
            default_local_network_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultLocalNetworkGatewayId").unwrap(),
            ),
            enable_bgp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableBgp").unwrap(),
            ),
            generation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("generation").unwrap(),
            ),
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
            private_ip_address_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateIpAddressEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            vpn_client_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpnClientConfigurations").unwrap(),
            ),
            vpn_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpnType").unwrap(),
            ),
        }
    }
}
