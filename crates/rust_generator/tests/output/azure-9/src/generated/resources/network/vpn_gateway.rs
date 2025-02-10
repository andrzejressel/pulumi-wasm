/// Manages a VPN Gateway within a Virtual Hub, which enables Site-to-Site communication.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpn_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpnGatewayArgs {
        /// Is BGP route translation for NAT on this VPN Gateway enabled? Defaults to `false`.
        #[builder(into, default)]
        pub bgp_route_translation_for_nat_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A `bgp_settings` block as defined below.
        #[builder(into, default)]
        pub bgp_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::VpnGatewayBgpSettings>,
        >,
        /// The Azure location where this VPN Gateway should be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Name which should be used for this VPN Gateway. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Name of the Resource Group in which this VPN Gateway should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Azure routing preference lets you to choose how your traffic routes between Azure and the internet. You can choose to route traffic either via the Microsoft network (default value, `Microsoft Network`), or via the ISP network (public internet, set to `Internet`). More context of the configuration can be found in the [Microsoft Docs](https://docs.microsoft.com/azure/virtual-wan/virtual-wan-site-to-site-portal#gateway) to create a VPN Gateway. Defaults to `Microsoft Network`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub routing_preference: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Scale Unit for this VPN Gateway. Defaults to `1`.
        #[builder(into, default)]
        pub scale_unit: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A mapping of tags to assign to the VPN Gateway.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Virtual Hub within which this VPN Gateway should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_hub_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpnGatewayResult {
        /// Is BGP route translation for NAT on this VPN Gateway enabled? Defaults to `false`.
        pub bgp_route_translation_for_nat_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// A `bgp_settings` block as defined below.
        pub bgp_settings: pulumi_gestalt_rust::Output<
            super::super::types::network::VpnGatewayBgpSettings,
        >,
        /// The Azure location where this VPN Gateway should be created. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The Name which should be used for this VPN Gateway. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Name of the Resource Group in which this VPN Gateway should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Azure routing preference lets you to choose how your traffic routes between Azure and the internet. You can choose to route traffic either via the Microsoft network (default value, `Microsoft Network`), or via the ISP network (public internet, set to `Internet`). More context of the configuration can be found in the [Microsoft Docs](https://docs.microsoft.com/azure/virtual-wan/virtual-wan-site-to-site-portal#gateway) to create a VPN Gateway. Defaults to `Microsoft Network`. Changing this forces a new resource to be created.
        pub routing_preference: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Scale Unit for this VPN Gateway. Defaults to `1`.
        pub scale_unit: pulumi_gestalt_rust::Output<Option<i32>>,
        /// A mapping of tags to assign to the VPN Gateway.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Virtual Hub within which this VPN Gateway should be created. Changing this forces a new resource to be created.
        pub virtual_hub_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpnGatewayArgs,
    ) -> VpnGatewayResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bgp_route_translation_for_nat_enabled_binding = args
            .bgp_route_translation_for_nat_enabled
            .get_output(context);
        let bgp_settings_binding = args.bgp_settings.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let routing_preference_binding = args.routing_preference.get_output(context);
        let scale_unit_binding = args.scale_unit.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let virtual_hub_id_binding = args.virtual_hub_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/vpnGateway:VpnGateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bgpRouteTranslationForNatEnabled".into(),
                    value: bgp_route_translation_for_nat_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bgpSettings".into(),
                    value: bgp_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routingPreference".into(),
                    value: routing_preference_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scaleUnit".into(),
                    value: scale_unit_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualHubId".into(),
                    value: virtual_hub_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpnGatewayResult {
            bgp_route_translation_for_nat_enabled: o
                .get_field("bgpRouteTranslationForNatEnabled"),
            bgp_settings: o.get_field("bgpSettings"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            routing_preference: o.get_field("routingPreference"),
            scale_unit: o.get_field("scaleUnit"),
            tags: o.get_field("tags"),
            virtual_hub_id: o.get_field("virtualHubId"),
        }
    }
}
