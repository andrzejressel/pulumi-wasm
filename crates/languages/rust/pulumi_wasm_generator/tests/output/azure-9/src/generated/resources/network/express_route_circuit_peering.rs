/// Manages an ExpressRoute Circuit Peering.
///
/// ## Example Usage
///
/// ### Creating A Microsoft Peering)
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: exprtTest
///       location: West Europe
///   exampleExpressRouteCircuit:
///     type: azure:network:ExpressRouteCircuit
///     name: example
///     properties:
///       name: expressRoute1
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       serviceProviderName: Equinix
///       peeringLocation: Silicon Valley
///       bandwidthInMbps: 50
///       sku:
///         tier: Standard
///         family: MeteredData
///       allowClassicOperations: false
///       tags:
///         environment: Production
///   exampleExpressRouteCircuitPeering:
///     type: azure:network:ExpressRouteCircuitPeering
///     name: example
///     properties:
///       peeringType: MicrosoftPeering
///       expressRouteCircuitName: ${exampleExpressRouteCircuit.name}
///       resourceGroupName: ${example.name}
///       peerAsn: 100
///       primaryPeerAddressPrefix: 123.0.0.0/30
///       secondaryPeerAddressPrefix: 123.0.0.4/30
///       ipv4Enabled: true
///       vlanId: 300
///       microsoftPeeringConfig:
///         advertisedPublicPrefixes:
///           - 123.1.0.0/24
///       ipv6:
///         primaryPeerAddressPrefix: 2002:db01::/126
///         secondaryPeerAddressPrefix: 2003:db01::/126
///         enabled: true
///         microsoftPeering:
///           advertisedPublicPrefixes:
///             - 2002:db01::/126
/// ```
///
///
/// ### Creating Azure Private Peering)
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: exprtTest
///       location: West Europe
///   exampleExpressRouteCircuit:
///     type: azure:network:ExpressRouteCircuit
///     name: example
///     properties:
///       name: expressRoute1
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       serviceProviderName: Equinix
///       peeringLocation: Silicon Valley
///       bandwidthInMbps: 50
///       sku:
///         tier: Standard
///         family: MeteredData
///       allowClassicOperations: false
///       tags:
///         environment: Production
///   exampleExpressRouteCircuitPeering:
///     type: azure:network:ExpressRouteCircuitPeering
///     name: example
///     properties:
///       peeringType: AzurePrivatePeering
///       expressRouteCircuitName: ${exampleExpressRouteCircuit.name}
///       resourceGroupName: ${example.name}
///       peerAsn: 100
///       primaryPeerAddressPrefix: 123.0.0.0/30
///       secondaryPeerAddressPrefix: 123.0.0.4/30
///       ipv4Enabled: true
///       vlanId: 300
///       ipv6:
///         primaryPeerAddressPrefix: 2002:db01::/126
///         secondaryPeerAddressPrefix: 2003:db01::/126
///         enabled: true
/// ```
///
/// ## Import
///
/// ExpressRoute Circuit Peerings can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/expressRouteCircuitPeering:ExpressRouteCircuitPeering peering1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/expressRouteCircuits/myExpressRoute/peerings/peering1
/// ```
///
pub mod express_route_circuit_peering {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExpressRouteCircuitPeeringArgs {
        /// The name of the ExpressRoute Circuit in which to create the Peering. Changing this forces a new resource to be created.
        #[builder(into)]
        pub express_route_circuit_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A boolean value indicating whether the IPv4 peering is enabled. Defaults to `true`.
        #[builder(into, default)]
        pub ipv4_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A `ipv6` block as defined below.
        #[builder(into, default)]
        pub ipv6: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::network::ExpressRouteCircuitPeeringIpv6>,
        >,
        /// A `microsoft_peering_config` block as defined below. Required when `peering_type` is set to `MicrosoftPeering` and config for IPv4.
        #[builder(into, default)]
        pub microsoft_peering_config: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::network::ExpressRouteCircuitPeeringMicrosoftPeeringConfig,
            >,
        >,
        /// The Either a 16-bit or a 32-bit ASN. Can either be public or private.
        #[builder(into, default)]
        pub peer_asn: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The type of the ExpressRoute Circuit Peering. Acceptable values include `AzurePrivatePeering`, `AzurePublicPeering` and `MicrosoftPeering`.
        ///
        /// > **NOTE:** only one Peering of each Type can be created. Attempting to create multiple peerings of the same type will overwrite the original peering.
        #[builder(into)]
        pub peering_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// A subnet for the primary link.
        #[builder(into, default)]
        pub primary_peer_address_prefix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Express Route Circuit Peering. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the Route Filter. Only available when `peering_type` is set to `MicrosoftPeering`.
        ///
        /// > **NOTE:** `ipv6` can be specified when `peering_type` is `MicrosoftPeering` or `AzurePrivatePeering`
        #[builder(into, default)]
        pub route_filter_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A subnet for the secondary link.
        #[builder(into, default)]
        pub secondary_peer_address_prefix: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// The shared key. Can be a maximum of 25 characters.
        #[builder(into, default)]
        pub shared_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A valid VLAN ID to establish this peering on.
        #[builder(into)]
        pub vlan_id: pulumi_wasm_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct ExpressRouteCircuitPeeringResult {
        /// The ASN used by Azure.
        pub azure_asn: pulumi_wasm_rust::Output<i32>,
        /// The name of the ExpressRoute Circuit in which to create the Peering. Changing this forces a new resource to be created.
        pub express_route_circuit_name: pulumi_wasm_rust::Output<String>,
        pub gateway_manager_etag: pulumi_wasm_rust::Output<String>,
        /// A boolean value indicating whether the IPv4 peering is enabled. Defaults to `true`.
        pub ipv4_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `ipv6` block as defined below.
        pub ipv6: pulumi_wasm_rust::Output<
            Option<super::super::types::network::ExpressRouteCircuitPeeringIpv6>,
        >,
        /// A `microsoft_peering_config` block as defined below. Required when `peering_type` is set to `MicrosoftPeering` and config for IPv4.
        pub microsoft_peering_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::network::ExpressRouteCircuitPeeringMicrosoftPeeringConfig,
            >,
        >,
        /// The Either a 16-bit or a 32-bit ASN. Can either be public or private.
        pub peer_asn: pulumi_wasm_rust::Output<i32>,
        /// The type of the ExpressRoute Circuit Peering. Acceptable values include `AzurePrivatePeering`, `AzurePublicPeering` and `MicrosoftPeering`.
        ///
        /// > **NOTE:** only one Peering of each Type can be created. Attempting to create multiple peerings of the same type will overwrite the original peering.
        pub peering_type: pulumi_wasm_rust::Output<String>,
        /// The Primary Port used by Azure for this Peering.
        pub primary_azure_port: pulumi_wasm_rust::Output<String>,
        /// A subnet for the primary link.
        pub primary_peer_address_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the Express Route Circuit Peering. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Route Filter. Only available when `peering_type` is set to `MicrosoftPeering`.
        ///
        /// > **NOTE:** `ipv6` can be specified when `peering_type` is `MicrosoftPeering` or `AzurePrivatePeering`
        pub route_filter_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The Secondary Port used by Azure for this Peering.
        pub secondary_azure_port: pulumi_wasm_rust::Output<String>,
        /// A subnet for the secondary link.
        pub secondary_peer_address_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// The shared key. Can be a maximum of 25 characters.
        pub shared_key: pulumi_wasm_rust::Output<Option<String>>,
        /// A valid VLAN ID to establish this peering on.
        pub vlan_id: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ExpressRouteCircuitPeeringArgs,
    ) -> ExpressRouteCircuitPeeringResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let express_route_circuit_name_binding = args
            .express_route_circuit_name
            .get_output(context)
            .get_inner();
        let ipv4_enabled_binding = args.ipv4_enabled.get_output(context).get_inner();
        let ipv6_binding = args.ipv6.get_output(context).get_inner();
        let microsoft_peering_config_binding = args
            .microsoft_peering_config
            .get_output(context)
            .get_inner();
        let peer_asn_binding = args.peer_asn.get_output(context).get_inner();
        let peering_type_binding = args.peering_type.get_output(context).get_inner();
        let primary_peer_address_prefix_binding = args
            .primary_peer_address_prefix
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let route_filter_id_binding = args
            .route_filter_id
            .get_output(context)
            .get_inner();
        let secondary_peer_address_prefix_binding = args
            .secondary_peer_address_prefix
            .get_output(context)
            .get_inner();
        let shared_key_binding = args.shared_key.get_output(context).get_inner();
        let vlan_id_binding = args.vlan_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/expressRouteCircuitPeering:ExpressRouteCircuitPeering"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "expressRouteCircuitName".into(),
                    value: &express_route_circuit_name_binding,
                },
                register_interface::ObjectField {
                    name: "ipv4Enabled".into(),
                    value: &ipv4_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6".into(),
                    value: &ipv6_binding,
                },
                register_interface::ObjectField {
                    name: "microsoftPeeringConfig".into(),
                    value: &microsoft_peering_config_binding,
                },
                register_interface::ObjectField {
                    name: "peerAsn".into(),
                    value: &peer_asn_binding,
                },
                register_interface::ObjectField {
                    name: "peeringType".into(),
                    value: &peering_type_binding,
                },
                register_interface::ObjectField {
                    name: "primaryPeerAddressPrefix".into(),
                    value: &primary_peer_address_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "routeFilterId".into(),
                    value: &route_filter_id_binding,
                },
                register_interface::ObjectField {
                    name: "secondaryPeerAddressPrefix".into(),
                    value: &secondary_peer_address_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "sharedKey".into(),
                    value: &shared_key_binding,
                },
                register_interface::ObjectField {
                    name: "vlanId".into(),
                    value: &vlan_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ExpressRouteCircuitPeeringResult {
            azure_asn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("azureAsn"),
            ),
            express_route_circuit_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("expressRouteCircuitName"),
            ),
            gateway_manager_etag: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("gatewayManagerEtag"),
            ),
            ipv4_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipv4Enabled"),
            ),
            ipv6: pulumi_wasm_rust::__private::into_domain(o.extract_field("ipv6")),
            microsoft_peering_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("microsoftPeeringConfig"),
            ),
            peer_asn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("peerAsn"),
            ),
            peering_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("peeringType"),
            ),
            primary_azure_port: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryAzurePort"),
            ),
            primary_peer_address_prefix: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryPeerAddressPrefix"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            route_filter_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("routeFilterId"),
            ),
            secondary_azure_port: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryAzurePort"),
            ),
            secondary_peer_address_prefix: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryPeerAddressPrefix"),
            ),
            shared_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sharedKey"),
            ),
            vlan_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("vlanId")),
        }
    }
}
