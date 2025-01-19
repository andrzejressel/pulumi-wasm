/// Manages a connection in an existing Virtual Network Gateway.
///
/// ## Example Usage
///
/// ### Site-to-Site connection
///
/// The following example shows a connection between an Azure virtual network
/// and an on-premises VPN device and network.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder().location("West US").name("test").build_struct(),
///     );
///     let examplePublicIp = public_ip::create(
///         "examplePublicIp",
///         PublicIpArgs::builder()
///             .allocation_method("Dynamic")
///             .location("${example.location}")
///             .name("test")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.1.0/24",])
///             .name("GatewaySubnet")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("test")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetworkGateway = virtual_network_gateway::create(
///         "exampleVirtualNetworkGateway",
///         VirtualNetworkGatewayArgs::builder()
///             .active_active(false)
///             .enable_bgp(false)
///             .ip_configurations(
///                 vec![
///                     VirtualNetworkGatewayIpConfiguration::builder()
///                     .privateIpAddressAllocation("Dynamic")
///                     .publicIpAddressId("${examplePublicIp.id}")
///                     .subnetId("${exampleSubnet.id}").build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("test")
///             .resource_group_name("${example.name}")
///             .sku("Basic")
///             .type_("Vpn")
///             .vpn_type("RouteBased")
///             .build_struct(),
///     );
///     let onpremise = local_network_gateway::create(
///         "onpremise",
///         LocalNetworkGatewayArgs::builder()
///             .address_spaces(vec!["10.1.1.0/24",])
///             .gateway_address("168.62.225.23")
///             .location("${example.location}")
///             .name("onpremise")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let onpremiseVirtualNetworkGatewayConnection = virtual_network_gateway_connection::create(
///         "onpremiseVirtualNetworkGatewayConnection",
///         VirtualNetworkGatewayConnectionArgs::builder()
///             .local_network_gateway_id("${onpremise.id}")
///             .location("${example.location}")
///             .name("onpremise")
///             .resource_group_name("${example.name}")
///             .shared_key("4-v3ry-53cr37-1p53c-5h4r3d-k3y")
///             .type_("IPsec")
///             .virtual_network_gateway_id("${exampleVirtualNetworkGateway.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### VNet-to-VNet connection
///
/// The following example shows a connection between two Azure virtual network
/// in different locations/regions.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let europe = resource_group::create(
///         "europe",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("europe")
///             .build_struct(),
///     );
///     let europeGateway = subnet::create(
///         "europeGateway",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.1.1.0/24",])
///             .name("GatewaySubnet")
///             .resource_group_name("${europe.name}")
///             .virtual_network_name("${europeVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let europePublicIp = public_ip::create(
///         "europePublicIp",
///         PublicIpArgs::builder()
///             .allocation_method("Dynamic")
///             .location("${europe.location}")
///             .name("europe")
///             .resource_group_name("${europe.name}")
///             .build_struct(),
///     );
///     let europeToUs = virtual_network_gateway_connection::create(
///         "europeToUs",
///         VirtualNetworkGatewayConnectionArgs::builder()
///             .location("${europe.location}")
///             .name("europe-to-us")
///             .peer_virtual_network_gateway_id("${usVirtualNetworkGateway.id}")
///             .resource_group_name("${europe.name}")
///             .shared_key("4-v3ry-53cr37-1p53c-5h4r3d-k3y")
///             .type_("Vnet2Vnet")
///             .virtual_network_gateway_id("${europeVirtualNetworkGateway.id}")
///             .build_struct(),
///     );
///     let europeVirtualNetwork = virtual_network::create(
///         "europeVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.1.0.0/16",])
///             .location("${europe.location}")
///             .name("europe")
///             .resource_group_name("${europe.name}")
///             .build_struct(),
///     );
///     let europeVirtualNetworkGateway = virtual_network_gateway::create(
///         "europeVirtualNetworkGateway",
///         VirtualNetworkGatewayArgs::builder()
///             .ip_configurations(
///                 vec![
///                     VirtualNetworkGatewayIpConfiguration::builder()
///                     .privateIpAddressAllocation("Dynamic")
///                     .publicIpAddressId("${europePublicIp.id}")
///                     .subnetId("${europeGateway.id}").build_struct(),
///                 ],
///             )
///             .location("${europe.location}")
///             .name("europe-gateway")
///             .resource_group_name("${europe.name}")
///             .sku("Basic")
///             .type_("Vpn")
///             .vpn_type("RouteBased")
///             .build_struct(),
///     );
///     let us = resource_group::create(
///         "us",
///         ResourceGroupArgs::builder().location("East US").name("us").build_struct(),
///     );
///     let usGateway = subnet::create(
///         "usGateway",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.1.0/24",])
///             .name("GatewaySubnet")
///             .resource_group_name("${us.name}")
///             .virtual_network_name("${usVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let usPublicIp = public_ip::create(
///         "usPublicIp",
///         PublicIpArgs::builder()
///             .allocation_method("Dynamic")
///             .location("${us.location}")
///             .name("us")
///             .resource_group_name("${us.name}")
///             .build_struct(),
///     );
///     let usToEurope = virtual_network_gateway_connection::create(
///         "usToEurope",
///         VirtualNetworkGatewayConnectionArgs::builder()
///             .location("${us.location}")
///             .name("us-to-europe")
///             .peer_virtual_network_gateway_id("${europeVirtualNetworkGateway.id}")
///             .resource_group_name("${us.name}")
///             .shared_key("4-v3ry-53cr37-1p53c-5h4r3d-k3y")
///             .type_("Vnet2Vnet")
///             .virtual_network_gateway_id("${usVirtualNetworkGateway.id}")
///             .build_struct(),
///     );
///     let usVirtualNetwork = virtual_network::create(
///         "usVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${us.location}")
///             .name("us")
///             .resource_group_name("${us.name}")
///             .build_struct(),
///     );
///     let usVirtualNetworkGateway = virtual_network_gateway::create(
///         "usVirtualNetworkGateway",
///         VirtualNetworkGatewayArgs::builder()
///             .ip_configurations(
///                 vec![
///                     VirtualNetworkGatewayIpConfiguration::builder()
///                     .privateIpAddressAllocation("Dynamic")
///                     .publicIpAddressId("${usPublicIp.id}").subnetId("${usGateway.id}")
///                     .build_struct(),
///                 ],
///             )
///             .location("${us.location}")
///             .name("us-gateway")
///             .resource_group_name("${us.name}")
///             .sku("Basic")
///             .type_("Vpn")
///             .vpn_type("RouteBased")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Virtual Network Gateway Connections can be imported using their `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/virtualNetworkGatewayConnection:VirtualNetworkGatewayConnection exampleConnection /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myGroup1/providers/Microsoft.Network/connections/myConnection1
/// ```
///
pub mod virtual_network_gateway_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualNetworkGatewayConnectionArgs {
        /// The authorization key associated with the Express Route Circuit. This field is required only if the type is an ExpressRoute connection.
        #[builder(into, default)]
        pub authorization_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Connection mode to use. Possible values are `Default`, `InitiatorOnly` and `ResponderOnly`. Defaults to `Default`. Changing this value will force a resource to be created.
        #[builder(into, default)]
        pub connection_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The IKE protocol version to use. Possible values are `IKEv1` and `IKEv2`, values are `IKEv1` and `IKEv2`. Defaults to `IKEv2`. Changing this forces a new resource to be created.
        /// > **Note:** Only valid for `IPSec` connections on virtual network gateways with SKU `VpnGw1`, `VpnGw2`, `VpnGw3`, `VpnGw1AZ`, `VpnGw2AZ` or `VpnGw3AZ`.
        #[builder(into, default)]
        pub connection_protocol: pulumi_wasm_rust::Output<Option<String>>,
        /// A `custom_bgp_addresses` block which is documented below.
        /// The block can only be used on `IPSec` / `activeactive` connections,
        /// For details about see [the relevant section in the Azure documentation](https://docs.microsoft.com/en-us/azure/vpn-gateway/vpn-gateway-howto-aws-bgp).
        #[builder(into, default)]
        pub custom_bgp_addresses: pulumi_wasm_rust::Output<
            Option<
                super::super::types::network::VirtualNetworkGatewayConnectionCustomBgpAddresses,
            >,
        >,
        /// The dead peer detection timeout of this connection in seconds. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub dpd_timeout_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// A list of the egress NAT Rule Ids.
        #[builder(into, default)]
        pub egress_nat_rule_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// If `true`, BGP (Border Gateway Protocol) is enabled for this connection. Defaults to `false`.
        #[builder(into, default)]
        pub enable_bgp: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Express Route Circuit when creating an ExpressRoute connection (i.e. when `type` is `ExpressRoute`). The Express Route Circuit can be in the same or in a different subscription. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub express_route_circuit_id: pulumi_wasm_rust::Output<Option<String>>,
        /// If `true`, data packets will bypass ExpressRoute Gateway for data forwarding This is only valid for ExpressRoute connections.
        #[builder(into, default)]
        pub express_route_gateway_bypass: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of the ingress NAT Rule Ids.
        #[builder(into, default)]
        pub ingress_nat_rule_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A `ipsec_policy` block which is documented below.
        /// Only a single policy can be defined for a connection. For details on
        /// custom policies refer to [the relevant section in the Azure documentation](https://docs.microsoft.com/azure/vpn-gateway/vpn-gateway-ipsecikepolicy-rm-powershell).
        #[builder(into, default)]
        pub ipsec_policy: pulumi_wasm_rust::Output<
            Option<
                super::super::types::network::VirtualNetworkGatewayConnectionIpsecPolicy,
            >,
        >,
        /// Use private local Azure IP for the connection. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub local_azure_ip_address_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the local network gateway when creating Site-to-Site connection (i.e. when `type` is `IPsec`).
        #[builder(into, default)]
        pub local_network_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The location/region where the connection is located. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the connection. Changing the name forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the peer virtual network gateway when creating a VNet-to-VNet connection (i.e. when `type` is `Vnet2Vnet`). The peer Virtual Network Gateway can be in the same or in a different subscription. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub peer_virtual_network_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Bypass the Express Route gateway when accessing private-links. When enabled `express_route_gateway_bypass` must be set to `true`. Defaults to `false`.
        #[builder(into, default)]
        pub private_link_fast_path_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the connection Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The routing weight. Defaults to `10`.
        #[builder(into, default)]
        pub routing_weight: pulumi_wasm_rust::Output<Option<i32>>,
        /// The shared IPSec key. A key could be provided if a Site-to-Site, VNet-to-VNet or ExpressRoute connection is created.
        #[builder(into, default)]
        pub shared_key: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// One or more `traffic_selector_policy` blocks which are documented below.
        /// A `traffic_selector_policy` allows to specify a traffic selector policy proposal to be used in a virtual network gateway connection.
        /// For details about traffic selectors refer to [the relevant section in the Azure documentation](https://docs.microsoft.com/azure/vpn-gateway/vpn-gateway-connect-multiple-policybased-rm-ps).
        #[builder(into, default)]
        pub traffic_selector_policy: pulumi_wasm_rust::Output<
            Option<
                super::super::types::network::VirtualNetworkGatewayConnectionTrafficSelectorPolicy,
            >,
        >,
        /// The type of connection. Valid options are `IPsec` (Site-to-Site), `ExpressRoute` (ExpressRoute), and `Vnet2Vnet` (VNet-to-VNet). Each connection type requires different mandatory arguments (refer to the examples above). Changing this forces a new resource to be created.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
        /// If `true`, policy-based traffic selectors are enabled for this connection. Enabling policy-based traffic selectors requires an `ipsec_policy` block. Defaults to `false`.
        #[builder(into, default)]
        pub use_policy_based_traffic_selectors: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Virtual Network Gateway in which the connection will be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_network_gateway_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualNetworkGatewayConnectionResult {
        /// The authorization key associated with the Express Route Circuit. This field is required only if the type is an ExpressRoute connection.
        pub authorization_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Connection mode to use. Possible values are `Default`, `InitiatorOnly` and `ResponderOnly`. Defaults to `Default`. Changing this value will force a resource to be created.
        pub connection_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The IKE protocol version to use. Possible values are `IKEv1` and `IKEv2`, values are `IKEv1` and `IKEv2`. Defaults to `IKEv2`. Changing this forces a new resource to be created.
        /// > **Note:** Only valid for `IPSec` connections on virtual network gateways with SKU `VpnGw1`, `VpnGw2`, `VpnGw3`, `VpnGw1AZ`, `VpnGw2AZ` or `VpnGw3AZ`.
        pub connection_protocol: pulumi_wasm_rust::Output<String>,
        /// A `custom_bgp_addresses` block which is documented below.
        /// The block can only be used on `IPSec` / `activeactive` connections,
        /// For details about see [the relevant section in the Azure documentation](https://docs.microsoft.com/en-us/azure/vpn-gateway/vpn-gateway-howto-aws-bgp).
        pub custom_bgp_addresses: pulumi_wasm_rust::Output<
            Option<
                super::super::types::network::VirtualNetworkGatewayConnectionCustomBgpAddresses,
            >,
        >,
        /// The dead peer detection timeout of this connection in seconds. Changing this forces a new resource to be created.
        pub dpd_timeout_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// A list of the egress NAT Rule Ids.
        pub egress_nat_rule_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// If `true`, BGP (Border Gateway Protocol) is enabled for this connection. Defaults to `false`.
        pub enable_bgp: pulumi_wasm_rust::Output<bool>,
        /// The ID of the Express Route Circuit when creating an ExpressRoute connection (i.e. when `type` is `ExpressRoute`). The Express Route Circuit can be in the same or in a different subscription. Changing this forces a new resource to be created.
        pub express_route_circuit_id: pulumi_wasm_rust::Output<Option<String>>,
        /// If `true`, data packets will bypass ExpressRoute Gateway for data forwarding This is only valid for ExpressRoute connections.
        pub express_route_gateway_bypass: pulumi_wasm_rust::Output<bool>,
        /// A list of the ingress NAT Rule Ids.
        pub ingress_nat_rule_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A `ipsec_policy` block which is documented below.
        /// Only a single policy can be defined for a connection. For details on
        /// custom policies refer to [the relevant section in the Azure documentation](https://docs.microsoft.com/azure/vpn-gateway/vpn-gateway-ipsecikepolicy-rm-powershell).
        pub ipsec_policy: pulumi_wasm_rust::Output<
            Option<
                super::super::types::network::VirtualNetworkGatewayConnectionIpsecPolicy,
            >,
        >,
        /// Use private local Azure IP for the connection. Changing this forces a new resource to be created.
        pub local_azure_ip_address_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the local network gateway when creating Site-to-Site connection (i.e. when `type` is `IPsec`).
        pub local_network_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The location/region where the connection is located. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the connection. Changing the name forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the peer virtual network gateway when creating a VNet-to-VNet connection (i.e. when `type` is `Vnet2Vnet`). The peer Virtual Network Gateway can be in the same or in a different subscription. Changing this forces a new resource to be created.
        pub peer_virtual_network_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Bypass the Express Route gateway when accessing private-links. When enabled `express_route_gateway_bypass` must be set to `true`. Defaults to `false`.
        pub private_link_fast_path_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the connection Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The routing weight. Defaults to `10`.
        pub routing_weight: pulumi_wasm_rust::Output<i32>,
        /// The shared IPSec key. A key could be provided if a Site-to-Site, VNet-to-VNet or ExpressRoute connection is created.
        pub shared_key: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// One or more `traffic_selector_policy` blocks which are documented below.
        /// A `traffic_selector_policy` allows to specify a traffic selector policy proposal to be used in a virtual network gateway connection.
        /// For details about traffic selectors refer to [the relevant section in the Azure documentation](https://docs.microsoft.com/azure/vpn-gateway/vpn-gateway-connect-multiple-policybased-rm-ps).
        pub traffic_selector_policy: pulumi_wasm_rust::Output<
            Option<
                super::super::types::network::VirtualNetworkGatewayConnectionTrafficSelectorPolicy,
            >,
        >,
        /// The type of connection. Valid options are `IPsec` (Site-to-Site), `ExpressRoute` (ExpressRoute), and `Vnet2Vnet` (VNet-to-VNet). Each connection type requires different mandatory arguments (refer to the examples above). Changing this forces a new resource to be created.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// If `true`, policy-based traffic selectors are enabled for this connection. Enabling policy-based traffic selectors requires an `ipsec_policy` block. Defaults to `false`.
        pub use_policy_based_traffic_selectors: pulumi_wasm_rust::Output<bool>,
        /// The ID of the Virtual Network Gateway in which the connection will be created. Changing this forces a new resource to be created.
        pub virtual_network_gateway_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VirtualNetworkGatewayConnectionArgs,
    ) -> VirtualNetworkGatewayConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authorization_key_binding = args.authorization_key.get_inner();
        let connection_mode_binding = args.connection_mode.get_inner();
        let connection_protocol_binding = args.connection_protocol.get_inner();
        let custom_bgp_addresses_binding = args.custom_bgp_addresses.get_inner();
        let dpd_timeout_seconds_binding = args.dpd_timeout_seconds.get_inner();
        let egress_nat_rule_ids_binding = args.egress_nat_rule_ids.get_inner();
        let enable_bgp_binding = args.enable_bgp.get_inner();
        let express_route_circuit_id_binding = args.express_route_circuit_id.get_inner();
        let express_route_gateway_bypass_binding = args
            .express_route_gateway_bypass
            .get_inner();
        let ingress_nat_rule_ids_binding = args.ingress_nat_rule_ids.get_inner();
        let ipsec_policy_binding = args.ipsec_policy.get_inner();
        let local_azure_ip_address_enabled_binding = args
            .local_azure_ip_address_enabled
            .get_inner();
        let local_network_gateway_id_binding = args.local_network_gateway_id.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let peer_virtual_network_gateway_id_binding = args
            .peer_virtual_network_gateway_id
            .get_inner();
        let private_link_fast_path_enabled_binding = args
            .private_link_fast_path_enabled
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let routing_weight_binding = args.routing_weight.get_inner();
        let shared_key_binding = args.shared_key.get_inner();
        let tags_binding = args.tags.get_inner();
        let traffic_selector_policy_binding = args.traffic_selector_policy.get_inner();
        let type__binding = args.type_.get_inner();
        let use_policy_based_traffic_selectors_binding = args
            .use_policy_based_traffic_selectors
            .get_inner();
        let virtual_network_gateway_id_binding = args
            .virtual_network_gateway_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/virtualNetworkGatewayConnection:VirtualNetworkGatewayConnection"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authorizationKey".into(),
                    value: &authorization_key_binding,
                },
                register_interface::ObjectField {
                    name: "connectionMode".into(),
                    value: &connection_mode_binding,
                },
                register_interface::ObjectField {
                    name: "connectionProtocol".into(),
                    value: &connection_protocol_binding,
                },
                register_interface::ObjectField {
                    name: "customBgpAddresses".into(),
                    value: &custom_bgp_addresses_binding,
                },
                register_interface::ObjectField {
                    name: "dpdTimeoutSeconds".into(),
                    value: &dpd_timeout_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "egressNatRuleIds".into(),
                    value: &egress_nat_rule_ids_binding,
                },
                register_interface::ObjectField {
                    name: "enableBgp".into(),
                    value: &enable_bgp_binding,
                },
                register_interface::ObjectField {
                    name: "expressRouteCircuitId".into(),
                    value: &express_route_circuit_id_binding,
                },
                register_interface::ObjectField {
                    name: "expressRouteGatewayBypass".into(),
                    value: &express_route_gateway_bypass_binding,
                },
                register_interface::ObjectField {
                    name: "ingressNatRuleIds".into(),
                    value: &ingress_nat_rule_ids_binding,
                },
                register_interface::ObjectField {
                    name: "ipsecPolicy".into(),
                    value: &ipsec_policy_binding,
                },
                register_interface::ObjectField {
                    name: "localAzureIpAddressEnabled".into(),
                    value: &local_azure_ip_address_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "localNetworkGatewayId".into(),
                    value: &local_network_gateway_id_binding,
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
                    name: "peerVirtualNetworkGatewayId".into(),
                    value: &peer_virtual_network_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "privateLinkFastPathEnabled".into(),
                    value: &private_link_fast_path_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "routingWeight".into(),
                    value: &routing_weight_binding,
                },
                register_interface::ObjectField {
                    name: "sharedKey".into(),
                    value: &shared_key_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "trafficSelectorPolicy".into(),
                    value: &traffic_selector_policy_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "usePolicyBasedTrafficSelectors".into(),
                    value: &use_policy_based_traffic_selectors_binding,
                },
                register_interface::ObjectField {
                    name: "virtualNetworkGatewayId".into(),
                    value: &virtual_network_gateway_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "authorizationKey".into(),
                },
                register_interface::ResultField {
                    name: "connectionMode".into(),
                },
                register_interface::ResultField {
                    name: "connectionProtocol".into(),
                },
                register_interface::ResultField {
                    name: "customBgpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "dpdTimeoutSeconds".into(),
                },
                register_interface::ResultField {
                    name: "egressNatRuleIds".into(),
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
                    name: "ingressNatRuleIds".into(),
                },
                register_interface::ResultField {
                    name: "ipsecPolicy".into(),
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
                    name: "routingWeight".into(),
                },
                register_interface::ResultField {
                    name: "sharedKey".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "trafficSelectorPolicy".into(),
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
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VirtualNetworkGatewayConnectionResult {
            authorization_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizationKey").unwrap(),
            ),
            connection_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionMode").unwrap(),
            ),
            connection_protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionProtocol").unwrap(),
            ),
            custom_bgp_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customBgpAddresses").unwrap(),
            ),
            dpd_timeout_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dpdTimeoutSeconds").unwrap(),
            ),
            egress_nat_rule_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("egressNatRuleIds").unwrap(),
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
            ingress_nat_rule_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ingressNatRuleIds").unwrap(),
            ),
            ipsec_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipsecPolicy").unwrap(),
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
            routing_weight: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routingWeight").unwrap(),
            ),
            shared_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sharedKey").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            traffic_selector_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trafficSelectorPolicy").unwrap(),
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
