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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod express_route_circuit_peering {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExpressRouteCircuitPeeringArgs {
        /// The name of the ExpressRoute Circuit in which to create the Peering. Changing this forces a new resource to be created.
        #[builder(into)]
        pub express_route_circuit_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A boolean value indicating whether the IPv4 peering is enabled. Defaults to `true`.
        #[builder(into, default)]
        pub ipv4_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `ipv6` block as defined below.
        #[builder(into, default)]
        pub ipv6: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::ExpressRouteCircuitPeeringIpv6>,
        >,
        /// A `microsoft_peering_config` block as defined below. Required when `peering_type` is set to `MicrosoftPeering` and config for IPv4.
        #[builder(into, default)]
        pub microsoft_peering_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::network::ExpressRouteCircuitPeeringMicrosoftPeeringConfig,
            >,
        >,
        /// The Either a 16-bit or a 32-bit ASN. Can either be public or private.
        #[builder(into, default)]
        pub peer_asn: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The type of the ExpressRoute Circuit Peering. Acceptable values include `AzurePrivatePeering`, `AzurePublicPeering` and `MicrosoftPeering`.
        ///
        /// > **NOTE:** only one Peering of each Type can be created. Attempting to create multiple peerings of the same type will overwrite the original peering.
        #[builder(into)]
        pub peering_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A subnet for the primary link.
        #[builder(into, default)]
        pub primary_peer_address_prefix: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of the resource group in which to create the Express Route Circuit Peering. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Route Filter. Only available when `peering_type` is set to `MicrosoftPeering`.
        ///
        /// > **NOTE:** `ipv6` can be specified when `peering_type` is `MicrosoftPeering` or `AzurePrivatePeering`
        #[builder(into, default)]
        pub route_filter_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A subnet for the secondary link.
        #[builder(into, default)]
        pub secondary_peer_address_prefix: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The shared key. Can be a maximum of 25 characters.
        #[builder(into, default)]
        pub shared_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A valid VLAN ID to establish this peering on.
        #[builder(into)]
        pub vlan_id: pulumi_gestalt_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct ExpressRouteCircuitPeeringResult {
        /// The ASN used by Azure.
        pub azure_asn: pulumi_gestalt_rust::Output<i32>,
        /// The name of the ExpressRoute Circuit in which to create the Peering. Changing this forces a new resource to be created.
        pub express_route_circuit_name: pulumi_gestalt_rust::Output<String>,
        pub gateway_manager_etag: pulumi_gestalt_rust::Output<String>,
        /// A boolean value indicating whether the IPv4 peering is enabled. Defaults to `true`.
        pub ipv4_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `ipv6` block as defined below.
        pub ipv6: pulumi_gestalt_rust::Output<
            Option<super::super::types::network::ExpressRouteCircuitPeeringIpv6>,
        >,
        /// A `microsoft_peering_config` block as defined below. Required when `peering_type` is set to `MicrosoftPeering` and config for IPv4.
        pub microsoft_peering_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::network::ExpressRouteCircuitPeeringMicrosoftPeeringConfig,
            >,
        >,
        /// The Either a 16-bit or a 32-bit ASN. Can either be public or private.
        pub peer_asn: pulumi_gestalt_rust::Output<i32>,
        /// The type of the ExpressRoute Circuit Peering. Acceptable values include `AzurePrivatePeering`, `AzurePublicPeering` and `MicrosoftPeering`.
        ///
        /// > **NOTE:** only one Peering of each Type can be created. Attempting to create multiple peerings of the same type will overwrite the original peering.
        pub peering_type: pulumi_gestalt_rust::Output<String>,
        /// The Primary Port used by Azure for this Peering.
        pub primary_azure_port: pulumi_gestalt_rust::Output<String>,
        /// A subnet for the primary link.
        pub primary_peer_address_prefix: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the Express Route Circuit Peering. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Route Filter. Only available when `peering_type` is set to `MicrosoftPeering`.
        ///
        /// > **NOTE:** `ipv6` can be specified when `peering_type` is `MicrosoftPeering` or `AzurePrivatePeering`
        pub route_filter_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Secondary Port used by Azure for this Peering.
        pub secondary_azure_port: pulumi_gestalt_rust::Output<String>,
        /// A subnet for the secondary link.
        pub secondary_peer_address_prefix: pulumi_gestalt_rust::Output<Option<String>>,
        /// The shared key. Can be a maximum of 25 characters.
        pub shared_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// A valid VLAN ID to establish this peering on.
        pub vlan_id: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExpressRouteCircuitPeeringArgs,
    ) -> ExpressRouteCircuitPeeringResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let express_route_circuit_name_binding = args
            .express_route_circuit_name
            .get_output(context);
        let ipv4_enabled_binding = args.ipv4_enabled.get_output(context);
        let ipv6_binding = args.ipv6.get_output(context);
        let microsoft_peering_config_binding = args
            .microsoft_peering_config
            .get_output(context);
        let peer_asn_binding = args.peer_asn.get_output(context);
        let peering_type_binding = args.peering_type.get_output(context);
        let primary_peer_address_prefix_binding = args
            .primary_peer_address_prefix
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let route_filter_id_binding = args.route_filter_id.get_output(context);
        let secondary_peer_address_prefix_binding = args
            .secondary_peer_address_prefix
            .get_output(context);
        let shared_key_binding = args.shared_key.get_output(context);
        let vlan_id_binding = args.vlan_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/expressRouteCircuitPeering:ExpressRouteCircuitPeering"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expressRouteCircuitName".into(),
                    value: &express_route_circuit_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipv4Enabled".into(),
                    value: &ipv4_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipv6".into(),
                    value: &ipv6_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "microsoftPeeringConfig".into(),
                    value: &microsoft_peering_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peerAsn".into(),
                    value: &peer_asn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peeringType".into(),
                    value: &peering_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "primaryPeerAddressPrefix".into(),
                    value: &primary_peer_address_prefix_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routeFilterId".into(),
                    value: &route_filter_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secondaryPeerAddressPrefix".into(),
                    value: &secondary_peer_address_prefix_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sharedKey".into(),
                    value: &shared_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vlanId".into(),
                    value: &vlan_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ExpressRouteCircuitPeeringResult {
            azure_asn: o.get_field("azureAsn"),
            express_route_circuit_name: o.get_field("expressRouteCircuitName"),
            gateway_manager_etag: o.get_field("gatewayManagerEtag"),
            ipv4_enabled: o.get_field("ipv4Enabled"),
            ipv6: o.get_field("ipv6"),
            microsoft_peering_config: o.get_field("microsoftPeeringConfig"),
            peer_asn: o.get_field("peerAsn"),
            peering_type: o.get_field("peeringType"),
            primary_azure_port: o.get_field("primaryAzurePort"),
            primary_peer_address_prefix: o.get_field("primaryPeerAddressPrefix"),
            resource_group_name: o.get_field("resourceGroupName"),
            route_filter_id: o.get_field("routeFilterId"),
            secondary_azure_port: o.get_field("secondaryAzurePort"),
            secondary_peer_address_prefix: o.get_field("secondaryPeerAddressPrefix"),
            shared_key: o.get_field("sharedKey"),
            vlan_id: o.get_field("vlanId"),
        }
    }
}
