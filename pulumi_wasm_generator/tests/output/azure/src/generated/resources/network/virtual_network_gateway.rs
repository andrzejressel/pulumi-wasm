/// Manages a Virtual Network Gateway to establish secure, cross-premises connectivity.
///
/// > **Note:** Please be aware that provisioning a Virtual Network Gateway takes a long time (between 30 minutes and 1 hour)
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
///         ResourceGroupArgs::builder().location("West Europe").name("test").build_struct(),
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
///                     .name("vnetGatewayConfig").privateIpAddressAllocation("Dynamic")
///                     .publicIpAddressId("${examplePublicIp.id}")
///                     .subnetId("${exampleSubnet.id}").build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("test")
///             .resource_group_name("${example.name}")
///             .sku("Basic")
///             .type_("Vpn")
///             .vpn_client_configuration(
///                 VirtualNetworkGatewayVpnClientConfiguration::builder()
///                     .addressSpaces(vec!["10.2.0.0/24",])
///                     .revokedCertificates(
///                         vec![
///                             VirtualNetworkGatewayVpnClientConfigurationRevokedCertificate::builder()
///                             .name("Verizon-Global-Root-CA")
///                             .thumbprint("912198EEF23DCAC40939312FEE97DD560BAE49B1")
///                             .build_struct(),
///                         ],
///                     )
///                     .rootCertificates(
///                         vec![
///                             VirtualNetworkGatewayVpnClientConfigurationRootCertificate::builder()
///                             .name("DigiCert-Federated-ID-Root-CA")
///                             .publicCertData("MIIDuzCCAqOgAwIBAgIQCHTZWCM+IlfFIRXIvyKSrjANBgkqhkiG9w0BAQsFADBn\nMQswCQYDVQQGEwJVUzEVMBMGA1UEChMMRGlnaUNlcnQgSW5jMRkwFwYDVQQLExB3\nd3cuZGlnaWNlcnQuY29tMSYwJAYDVQQDEx1EaWdpQ2VydCBGZWRlcmF0ZWQgSUQg\nUm9vdCBDQTAeFw0xMzAxMTUxMjAwMDBaFw0zMzAxMTUxMjAwMDBaMGcxCzAJBgNV\nBAYTAlVTMRUwEwYDVQQKEwxEaWdpQ2VydCBJbmMxGTAXBgNVBAsTEHd3dy5kaWdp\nY2VydC5jb20xJjAkBgNVBAMTHURpZ2lDZXJ0IEZlZGVyYXRlZCBJRCBSb290IENB\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAvAEB4pcCqnNNOWE6Ur5j\nQPUH+1y1F9KdHTRSza6k5iDlXq1kGS1qAkuKtw9JsiNRrjltmFnzMZRBbX8Tlfl8\nzAhBmb6dDduDGED01kBsTkgywYPxXVTKec0WxYEEF0oMn4wSYNl0lt2eJAKHXjNf\nGTwiibdP8CUR2ghSM2sUTI8Nt1Omfc4SMHhGhYD64uJMbX98THQ/4LMGuYegou+d\nGTiahfHtjn7AboSEknwAMJHCh5RlYZZ6B1O4QbKJ+34Q0eKgnI3X6Vc9u0zf6DH8\nDk+4zQDYRRTqTnVO3VT8jzqDlCRuNtq6YvryOWN74/dq8LQhUnXHvFyrsdMaE1X2\nDwIDAQABo2MwYTAPBgNVHRMBAf8EBTADAQH/MA4GA1UdDwEB/wQEAwIBhjAdBgNV\nHQ4EFgQUGRdkFnbGt1EWjKwbUne+5OaZvRYwHwYDVR0jBBgwFoAUGRdkFnbGt1EW\njKwbUne+5OaZvRYwDQYJKoZIhvcNAQELBQADggEBAHcqsHkrjpESqfuVTRiptJfP\n9JbdtWqRTmOf6uJi2c8YVqI6XlKXsD8C1dUUaaHKLUJzvKiazibVuBwMIT84AyqR\nQELn3e0BtgEymEygMU569b01ZPxoFSnNXc7qDZBDef8WfqAV/sxkTi8L9BkmFYfL\nuGLOhRJOFprPdoDIUBB+tmCl3oDcBy3vnUeOEioz8zAkprcb3GHwHAK+vHmmfgcn\nWsfMLH4JCLa/tRYL+Rw/N3ybCkDp00s0WUZ+AoDywSl0Q/ZEnNY0MsFiw6LyIdbq\nM/s/1JRtO3bDSzD9TazRVzn2oBqzSa8VgIo5C1nOnoAKJTlsClJKvIhnRlaLQqk=\n")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .vpn_type("RouteBased")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Virtual Network Gateways can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/virtualNetworkGateway:VirtualNetworkGateway exampleGateway /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myGroup1/providers/Microsoft.Network/virtualNetworkGateways/myGateway1
/// ```
///
pub mod virtual_network_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualNetworkGatewayArgs {
        /// If `true`, an active-active Virtual Network Gateway will be created. An active-active gateway requires a `HighPerformance` or an `UltraPerformance` SKU. If `false`, an active-standby gateway will be created. Defaults to `false`.
        #[builder(into, default)]
        pub active_active: pulumi_wasm_rust::Output<Option<bool>>,
        /// Is BGP Route Translation for NAT enabled? Defaults to `false`.
        #[builder(into, default)]
        pub bgp_route_translation_for_nat_enabled: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// A `bgp_settings` block which is documented below. In this block the BGP specific settings can be defined.
        #[builder(into, default)]
        pub bgp_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::network::VirtualNetworkGatewayBgpSettings>,
        >,
        /// A `custom_route` block as defined below. Specifies a custom routes address space for a virtual network gateway and a VpnClient.
        #[builder(into, default)]
        pub custom_route: pulumi_wasm_rust::Output<
            Option<super::super::types::network::VirtualNetworkGatewayCustomRoute>,
        >,
        /// The ID of the local network gateway through which outbound Internet traffic from the virtual network in which the gateway is created will be routed (*forced tunnelling*). Refer to the [Azure documentation on forced tunnelling](https://docs.microsoft.com/azure/vpn-gateway/vpn-gateway-forced-tunneling-rm). If not specified, forced tunnelling is disabled.
        #[builder(into, default)]
        pub default_local_network_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Is DNS forwarding enabled?
        #[builder(into, default)]
        pub dns_forwarding_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the Edge Zone within the Azure Region where this Virtual Network Gateway should exist. Changing this forces a new Virtual Network Gateway to be created.
        #[builder(into, default)]
        pub edge_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// If `true`, BGP (Border Gateway Protocol) will be enabled for this Virtual Network Gateway. Defaults to `false`.
        #[builder(into, default)]
        pub enable_bgp: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Generation of the Virtual Network gateway. Possible values include `Generation1`, `Generation2` or `None`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The available values depend on the `type` and `sku` arguments - where `Generation2` is only value for a `sku` larger than `VpnGw2` or `VpnGw2AZ`.
        #[builder(into, default)]
        pub generation: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more (up to 3) `ip_configuration` blocks documented below. Changing this forces a new resource to be created.
        /// An active-standby gateway requires exactly one `ip_configuration` block,
        /// an active-active gateway requires exactly two `ip_configuration` blocks whereas
        /// an active-active zone redundant gateway with P2S configuration requires exactly three `ip_configuration` blocks.
        #[builder(into)]
        pub ip_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::VirtualNetworkGatewayIpConfiguration>,
        >,
        /// Is IP Sec Replay Protection enabled? Defaults to `true`.
        #[builder(into, default)]
        pub ip_sec_replay_protection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The location/region where the Virtual Network Gateway is located. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Virtual Network Gateway. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `policy_group` blocks as defined below.
        #[builder(into, default)]
        pub policy_groups: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::VirtualNetworkGatewayPolicyGroup>>,
        >,
        /// Should private IP be enabled on this gateway for connections? Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub private_ip_address_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Is remote vnet traffic that is used to configure this gateway to accept traffic from other Azure Virtual Networks enabled? Defaults to `false`.
        #[builder(into, default)]
        pub remote_vnet_traffic_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the Virtual Network Gateway. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Configuration of the size and capacity of the virtual network gateway. Valid options are `Basic`, `Standard`, `HighPerformance`, `UltraPerformance`, `ErGw1AZ`, `ErGw2AZ`, `ErGw3AZ`, `VpnGw1`, `VpnGw2`, `VpnGw3`, `VpnGw4`,`VpnGw5`, `VpnGw1AZ`, `VpnGw2AZ`, `VpnGw3AZ`,`VpnGw4AZ` and `VpnGw5AZ` and depend on the `type`, `vpn_type` and `generation` arguments. A `PolicyBased` gateway only supports the `Basic` SKU. Further, the `UltraPerformance` SKU is only supported by an `ExpressRoute` gateway.
        ///
        /// > **NOTE:** To build a UltraPerformance ExpressRoute Virtual Network gateway, the associated Public IP needs to be SKU "Basic" not "Standard"
        ///
        /// > **NOTE:** Not all SKUs (e.g. `ErGw1AZ`) are available in all regions. If you see `StatusCode=400 -- Original Error: Code="InvalidGatewaySkuSpecifiedForGatewayDeploymentType"` please try another region.
        #[builder(into)]
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of the Virtual Network Gateway. Valid options are `Vpn` or `ExpressRoute`. Changing the type forces a new resource to be created.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Is remote vnet traffic that is used to configure this gateway to accept traffic from remote Virtual WAN networks enabled? Defaults to `false`.
        #[builder(into, default)]
        pub virtual_wan_traffic_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `vpn_client_configuration` block which is documented below. In this block the Virtual Network Gateway can be configured to accept IPSec point-to-site connections.
        #[builder(into, default)]
        pub vpn_client_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::network::VirtualNetworkGatewayVpnClientConfiguration,
            >,
        >,
        /// The routing type of the Virtual Network Gateway. Valid options are `RouteBased` or `PolicyBased`. Defaults to `RouteBased`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub vpn_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VirtualNetworkGatewayResult {
        /// If `true`, an active-active Virtual Network Gateway will be created. An active-active gateway requires a `HighPerformance` or an `UltraPerformance` SKU. If `false`, an active-standby gateway will be created. Defaults to `false`.
        pub active_active: pulumi_wasm_rust::Output<bool>,
        /// Is BGP Route Translation for NAT enabled? Defaults to `false`.
        pub bgp_route_translation_for_nat_enabled: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// A `bgp_settings` block which is documented below. In this block the BGP specific settings can be defined.
        pub bgp_settings: pulumi_wasm_rust::Output<
            super::super::types::network::VirtualNetworkGatewayBgpSettings,
        >,
        /// A `custom_route` block as defined below. Specifies a custom routes address space for a virtual network gateway and a VpnClient.
        pub custom_route: pulumi_wasm_rust::Output<
            Option<super::super::types::network::VirtualNetworkGatewayCustomRoute>,
        >,
        /// The ID of the local network gateway through which outbound Internet traffic from the virtual network in which the gateway is created will be routed (*forced tunnelling*). Refer to the [Azure documentation on forced tunnelling](https://docs.microsoft.com/azure/vpn-gateway/vpn-gateway-forced-tunneling-rm). If not specified, forced tunnelling is disabled.
        pub default_local_network_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Is DNS forwarding enabled?
        pub dns_forwarding_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the Edge Zone within the Azure Region where this Virtual Network Gateway should exist. Changing this forces a new Virtual Network Gateway to be created.
        pub edge_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// If `true`, BGP (Border Gateway Protocol) will be enabled for this Virtual Network Gateway. Defaults to `false`.
        pub enable_bgp: pulumi_wasm_rust::Output<bool>,
        /// The Generation of the Virtual Network gateway. Possible values include `Generation1`, `Generation2` or `None`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The available values depend on the `type` and `sku` arguments - where `Generation2` is only value for a `sku` larger than `VpnGw2` or `VpnGw2AZ`.
        pub generation: pulumi_wasm_rust::Output<String>,
        /// One or more (up to 3) `ip_configuration` blocks documented below. Changing this forces a new resource to be created.
        /// An active-standby gateway requires exactly one `ip_configuration` block,
        /// an active-active gateway requires exactly two `ip_configuration` blocks whereas
        /// an active-active zone redundant gateway with P2S configuration requires exactly three `ip_configuration` blocks.
        pub ip_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::VirtualNetworkGatewayIpConfiguration>,
        >,
        /// Is IP Sec Replay Protection enabled? Defaults to `true`.
        pub ip_sec_replay_protection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The location/region where the Virtual Network Gateway is located. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the Virtual Network Gateway. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `policy_group` blocks as defined below.
        pub policy_groups: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::VirtualNetworkGatewayPolicyGroup>>,
        >,
        /// Should private IP be enabled on this gateway for connections? Changing this forces a new resource to be created.
        pub private_ip_address_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Is remote vnet traffic that is used to configure this gateway to accept traffic from other Azure Virtual Networks enabled? Defaults to `false`.
        pub remote_vnet_traffic_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the Virtual Network Gateway. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Configuration of the size and capacity of the virtual network gateway. Valid options are `Basic`, `Standard`, `HighPerformance`, `UltraPerformance`, `ErGw1AZ`, `ErGw2AZ`, `ErGw3AZ`, `VpnGw1`, `VpnGw2`, `VpnGw3`, `VpnGw4`,`VpnGw5`, `VpnGw1AZ`, `VpnGw2AZ`, `VpnGw3AZ`,`VpnGw4AZ` and `VpnGw5AZ` and depend on the `type`, `vpn_type` and `generation` arguments. A `PolicyBased` gateway only supports the `Basic` SKU. Further, the `UltraPerformance` SKU is only supported by an `ExpressRoute` gateway.
        ///
        /// > **NOTE:** To build a UltraPerformance ExpressRoute Virtual Network gateway, the associated Public IP needs to be SKU "Basic" not "Standard"
        ///
        /// > **NOTE:** Not all SKUs (e.g. `ErGw1AZ`) are available in all regions. If you see `StatusCode=400 -- Original Error: Code="InvalidGatewaySkuSpecifiedForGatewayDeploymentType"` please try another region.
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of the Virtual Network Gateway. Valid options are `Vpn` or `ExpressRoute`. Changing the type forces a new resource to be created.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Is remote vnet traffic that is used to configure this gateway to accept traffic from remote Virtual WAN networks enabled? Defaults to `false`.
        pub virtual_wan_traffic_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `vpn_client_configuration` block which is documented below. In this block the Virtual Network Gateway can be configured to accept IPSec point-to-site connections.
        pub vpn_client_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::network::VirtualNetworkGatewayVpnClientConfiguration,
            >,
        >,
        /// The routing type of the Virtual Network Gateway. Valid options are `RouteBased` or `PolicyBased`. Defaults to `RouteBased`. Changing this forces a new resource to be created.
        pub vpn_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VirtualNetworkGatewayArgs,
    ) -> VirtualNetworkGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let active_active_binding = args.active_active.get_inner();
        let bgp_route_translation_for_nat_enabled_binding = args
            .bgp_route_translation_for_nat_enabled
            .get_inner();
        let bgp_settings_binding = args.bgp_settings.get_inner();
        let custom_route_binding = args.custom_route.get_inner();
        let default_local_network_gateway_id_binding = args
            .default_local_network_gateway_id
            .get_inner();
        let dns_forwarding_enabled_binding = args.dns_forwarding_enabled.get_inner();
        let edge_zone_binding = args.edge_zone.get_inner();
        let enable_bgp_binding = args.enable_bgp.get_inner();
        let generation_binding = args.generation.get_inner();
        let ip_configurations_binding = args.ip_configurations.get_inner();
        let ip_sec_replay_protection_enabled_binding = args
            .ip_sec_replay_protection_enabled
            .get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let policy_groups_binding = args.policy_groups.get_inner();
        let private_ip_address_enabled_binding = args
            .private_ip_address_enabled
            .get_inner();
        let remote_vnet_traffic_enabled_binding = args
            .remote_vnet_traffic_enabled
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sku_binding = args.sku.get_inner();
        let tags_binding = args.tags.get_inner();
        let type__binding = args.type_.get_inner();
        let virtual_wan_traffic_enabled_binding = args
            .virtual_wan_traffic_enabled
            .get_inner();
        let vpn_client_configuration_binding = args.vpn_client_configuration.get_inner();
        let vpn_type_binding = args.vpn_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/virtualNetworkGateway:VirtualNetworkGateway".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "activeActive".into(),
                    value: &active_active_binding,
                },
                register_interface::ObjectField {
                    name: "bgpRouteTranslationForNatEnabled".into(),
                    value: &bgp_route_translation_for_nat_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "bgpSettings".into(),
                    value: &bgp_settings_binding,
                },
                register_interface::ObjectField {
                    name: "customRoute".into(),
                    value: &custom_route_binding,
                },
                register_interface::ObjectField {
                    name: "defaultLocalNetworkGatewayId".into(),
                    value: &default_local_network_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "dnsForwardingEnabled".into(),
                    value: &dns_forwarding_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "edgeZone".into(),
                    value: &edge_zone_binding,
                },
                register_interface::ObjectField {
                    name: "enableBgp".into(),
                    value: &enable_bgp_binding,
                },
                register_interface::ObjectField {
                    name: "generation".into(),
                    value: &generation_binding,
                },
                register_interface::ObjectField {
                    name: "ipConfigurations".into(),
                    value: &ip_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "ipSecReplayProtectionEnabled".into(),
                    value: &ip_sec_replay_protection_enabled_binding,
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
                    name: "policyGroups".into(),
                    value: &policy_groups_binding,
                },
                register_interface::ObjectField {
                    name: "privateIpAddressEnabled".into(),
                    value: &private_ip_address_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "remoteVnetTrafficEnabled".into(),
                    value: &remote_vnet_traffic_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "virtualWanTrafficEnabled".into(),
                    value: &virtual_wan_traffic_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "vpnClientConfiguration".into(),
                    value: &vpn_client_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "vpnType".into(),
                    value: &vpn_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "activeActive".into(),
                },
                register_interface::ResultField {
                    name: "bgpRouteTranslationForNatEnabled".into(),
                },
                register_interface::ResultField {
                    name: "bgpSettings".into(),
                },
                register_interface::ResultField {
                    name: "customRoute".into(),
                },
                register_interface::ResultField {
                    name: "defaultLocalNetworkGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "dnsForwardingEnabled".into(),
                },
                register_interface::ResultField {
                    name: "edgeZone".into(),
                },
                register_interface::ResultField {
                    name: "enableBgp".into(),
                },
                register_interface::ResultField {
                    name: "generation".into(),
                },
                register_interface::ResultField {
                    name: "ipConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "ipSecReplayProtectionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "policyGroups".into(),
                },
                register_interface::ResultField {
                    name: "privateIpAddressEnabled".into(),
                },
                register_interface::ResultField {
                    name: "remoteVnetTrafficEnabled".into(),
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
                    name: "virtualWanTrafficEnabled".into(),
                },
                register_interface::ResultField {
                    name: "vpnClientConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "vpnType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VirtualNetworkGatewayResult {
            active_active: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("activeActive").unwrap(),
            ),
            bgp_route_translation_for_nat_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bgpRouteTranslationForNatEnabled").unwrap(),
            ),
            bgp_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bgpSettings").unwrap(),
            ),
            custom_route: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customRoute").unwrap(),
            ),
            default_local_network_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultLocalNetworkGatewayId").unwrap(),
            ),
            dns_forwarding_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsForwardingEnabled").unwrap(),
            ),
            edge_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("edgeZone").unwrap(),
            ),
            enable_bgp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableBgp").unwrap(),
            ),
            generation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("generation").unwrap(),
            ),
            ip_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipConfigurations").unwrap(),
            ),
            ip_sec_replay_protection_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipSecReplayProtectionEnabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            policy_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyGroups").unwrap(),
            ),
            private_ip_address_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateIpAddressEnabled").unwrap(),
            ),
            remote_vnet_traffic_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("remoteVnetTrafficEnabled").unwrap(),
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
            virtual_wan_traffic_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualWanTrafficEnabled").unwrap(),
            ),
            vpn_client_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpnClientConfiguration").unwrap(),
            ),
            vpn_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpnType").unwrap(),
            ),
        }
    }
}