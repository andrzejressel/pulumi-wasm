/// Manages a VPN Gateway within a Virtual Hub, which enables Site-to-Site communication.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleVirtualHub = virtual_hub::create(
///         "exampleVirtualHub",
///         VirtualHubArgs::builder()
///             .address_prefix("10.0.1.0/24")
///             .location("${example.location}")
///             .name("example-hub")
///             .resource_group_name("${example.name}")
///             .virtual_wan_id("${exampleVirtualWan.id}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("example-network")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleVirtualWan = virtual_wan::create(
///         "exampleVirtualWan",
///         VirtualWanArgs::builder()
///             .location("${example.location}")
///             .name("example-vwan")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleVpnGateway = vpn_gateway::create(
///         "exampleVpnGateway",
///         VpnGatewayArgs::builder()
///             .location("${example.location}")
///             .name("example-vpng")
///             .resource_group_name("${example.name}")
///             .virtual_hub_id("${exampleVirtualHub.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// VPN Gateways can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/vpnGateway:VpnGateway gateway1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/vpnGateways/gateway1
/// ```
///
pub mod vpn_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpnGatewayArgs {
        /// Is BGP route translation for NAT on this VPN Gateway enabled? Defaults to `false`.
        #[builder(into, default)]
        pub bgp_route_translation_for_nat_enabled: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// A `bgp_settings` block as defined below.
        #[builder(into, default)]
        pub bgp_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::network::VpnGatewayBgpSettings>,
        >,
        /// The Azure location where this VPN Gateway should be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The Name which should be used for this VPN Gateway. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Name of the Resource Group in which this VPN Gateway should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Azure routing preference lets you to choose how your traffic routes between Azure and the internet. You can choose to route traffic either via the Microsoft network (default value, `Microsoft Network`), or via the ISP network (public internet, set to `Internet`). More context of the configuration can be found in the [Microsoft Docs](https://docs.microsoft.com/azure/virtual-wan/virtual-wan-site-to-site-portal#gateway) to create a VPN Gateway. Defaults to `Microsoft Network`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub routing_preference: pulumi_wasm_rust::Output<Option<String>>,
        /// The Scale Unit for this VPN Gateway. Defaults to `1`.
        #[builder(into, default)]
        pub scale_unit: pulumi_wasm_rust::Output<Option<i32>>,
        /// A mapping of tags to assign to the VPN Gateway.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Virtual Hub within which this VPN Gateway should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_hub_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VpnGatewayResult {
        /// Is BGP route translation for NAT on this VPN Gateway enabled? Defaults to `false`.
        pub bgp_route_translation_for_nat_enabled: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// A `bgp_settings` block as defined below.
        pub bgp_settings: pulumi_wasm_rust::Output<
            super::super::types::network::VpnGatewayBgpSettings,
        >,
        /// The Azure location where this VPN Gateway should be created. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The Name which should be used for this VPN Gateway. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Name of the Resource Group in which this VPN Gateway should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Azure routing preference lets you to choose how your traffic routes between Azure and the internet. You can choose to route traffic either via the Microsoft network (default value, `Microsoft Network`), or via the ISP network (public internet, set to `Internet`). More context of the configuration can be found in the [Microsoft Docs](https://docs.microsoft.com/azure/virtual-wan/virtual-wan-site-to-site-portal#gateway) to create a VPN Gateway. Defaults to `Microsoft Network`. Changing this forces a new resource to be created.
        pub routing_preference: pulumi_wasm_rust::Output<Option<String>>,
        /// The Scale Unit for this VPN Gateway. Defaults to `1`.
        pub scale_unit: pulumi_wasm_rust::Output<Option<i32>>,
        /// A mapping of tags to assign to the VPN Gateway.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Virtual Hub within which this VPN Gateway should be created. Changing this forces a new resource to be created.
        pub virtual_hub_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VpnGatewayArgs) -> VpnGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bgp_route_translation_for_nat_enabled_binding = args
            .bgp_route_translation_for_nat_enabled
            .get_inner();
        let bgp_settings_binding = args.bgp_settings.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let routing_preference_binding = args.routing_preference.get_inner();
        let scale_unit_binding = args.scale_unit.get_inner();
        let tags_binding = args.tags.get_inner();
        let virtual_hub_id_binding = args.virtual_hub_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/vpnGateway:VpnGateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bgpRouteTranslationForNatEnabled".into(),
                    value: &bgp_route_translation_for_nat_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "bgpSettings".into(),
                    value: &bgp_settings_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "routingPreference".into(),
                    value: &routing_preference_binding,
                },
                register_interface::ObjectField {
                    name: "scaleUnit".into(),
                    value: &scale_unit_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "virtualHubId".into(),
                    value: &virtual_hub_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bgpRouteTranslationForNatEnabled".into(),
                },
                register_interface::ResultField {
                    name: "bgpSettings".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "routingPreference".into(),
                },
                register_interface::ResultField {
                    name: "scaleUnit".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "virtualHubId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpnGatewayResult {
            bgp_route_translation_for_nat_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bgpRouteTranslationForNatEnabled").unwrap(),
            ),
            bgp_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bgpSettings").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            routing_preference: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routingPreference").unwrap(),
            ),
            scale_unit: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scaleUnit").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            virtual_hub_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualHubId").unwrap(),
            ),
        }
    }
}
