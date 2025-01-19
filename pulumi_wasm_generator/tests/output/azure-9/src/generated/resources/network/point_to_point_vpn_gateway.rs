/// Manages a Point-to-Site VPN Gateway.
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
///     let examplePointToPointVpnGateway = point_to_point_vpn_gateway::create(
///         "examplePointToPointVpnGateway",
///         PointToPointVpnGatewayArgs::builder()
///             .connection_configurations(
///                 vec![
///                     PointToPointVpnGatewayConnectionConfiguration::builder()
///                     .name("example-gateway-config")
///                     .vpnClientAddressPool(PointToPointVpnGatewayConnectionConfigurationVpnClientAddressPool::builder()
///                     .addressPrefixes(vec!["10.0.2.0/24",]).build_struct())
///                     .build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("example-vpn-gateway")
///             .resource_group_name("${example.name}")
///             .scale_unit(1)
///             .virtual_hub_id("${exampleVirtualHub.id}")
///             .vpn_server_configuration_id("${exampleVpnServerConfiguration.id}")
///             .build_struct(),
///     );
///     let exampleVirtualHub = virtual_hub::create(
///         "exampleVirtualHub",
///         VirtualHubArgs::builder()
///             .address_prefix("10.0.0.0/23")
///             .location("${example.location}")
///             .name("example-virtualhub")
///             .resource_group_name("${example.name}")
///             .virtual_wan_id("${exampleVirtualWan.id}")
///             .build_struct(),
///     );
///     let exampleVirtualWan = virtual_wan::create(
///         "exampleVirtualWan",
///         VirtualWanArgs::builder()
///             .location("${example.location}")
///             .name("example-virtualwan")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleVpnServerConfiguration = vpn_server_configuration::create(
///         "exampleVpnServerConfiguration",
///         VpnServerConfigurationArgs::builder()
///             .client_root_certificates(
///                 vec![
///                     VpnServerConfigurationClientRootCertificate::builder()
///                     .name("DigiCert-Federated-ID-Root-CA")
///                     .publicCertData("MIIDuzCCAqOgAwIBAgIQCHTZWCM+IlfFIRXIvyKSrjANBgkqhkiG9w0BAQsFADBn\nMQswCQYDVQQGEwJVUzEVMBMGA1UEChMMRGlnaUNlcnQgSW5jMRkwFwYDVQQLExB3\nd3cuZGlnaWNlcnQuY29tMSYwJAYDVQQDEx1EaWdpQ2VydCBGZWRlcmF0ZWQgSUQg\nUm9vdCBDQTAeFw0xMzAxMTUxMjAwMDBaFw0zMzAxMTUxMjAwMDBaMGcxCzAJBgNV\nBAYTAlVTMRUwEwYDVQQKEwxEaWdpQ2VydCBJbmMxGTAXBgNVBAsTEHd3dy5kaWdp\nY2VydC5jb20xJjAkBgNVBAMTHURpZ2lDZXJ0IEZlZGVyYXRlZCBJRCBSb290IENB\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAvAEB4pcCqnNNOWE6Ur5j\nQPUH+1y1F9KdHTRSza6k5iDlXq1kGS1qAkuKtw9JsiNRrjltmFnzMZRBbX8Tlfl8\nzAhBmb6dDduDGED01kBsTkgywYPxXVTKec0WxYEEF0oMn4wSYNl0lt2eJAKHXjNf\nGTwiibdP8CUR2ghSM2sUTI8Nt1Omfc4SMHhGhYD64uJMbX98THQ/4LMGuYegou+d\nGTiahfHtjn7AboSEknwAMJHCh5RlYZZ6B1O4QbKJ+34Q0eKgnI3X6Vc9u0zf6DH8\nDk+4zQDYRRTqTnVO3VT8jzqDlCRuNtq6YvryOWN74/dq8LQhUnXHvFyrsdMaE1X2\nDwIDAQABo2MwYTAPBgNVHRMBAf8EBTADAQH/MA4GA1UdDwEB/wQEAwIBhjAdBgNV\nHQ4EFgQUGRdkFnbGt1EWjKwbUne+5OaZvRYwHwYDVR0jBBgwFoAUGRdkFnbGt1EW\njKwbUne+5OaZvRYwDQYJKoZIhvcNAQELBQADggEBAHcqsHkrjpESqfuVTRiptJfP\n9JbdtWqRTmOf6uJi2c8YVqI6XlKXsD8C1dUUaaHKLUJzvKiazibVuBwMIT84AyqR\nQELn3e0BtgEymEygMU569b01ZPxoFSnNXc7qDZBDef8WfqAV/sxkTi8L9BkmFYfL\nuGLOhRJOFprPdoDIUBB+tmCl3oDcBy3vnUeOEioz8zAkprcb3GHwHAK+vHmmfgcn\nWsfMLH4JCLa/tRYL+Rw/N3ybCkDp00s0WUZ+AoDywSl0Q/ZEnNY0MsFiw6LyIdbq\nM/s/1JRtO3bDSzD9TazRVzn2oBqzSa8VgIo5C1nOnoAKJTlsClJKvIhnRlaLQqk=\n")
///                     .build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("example-config")
///             .resource_group_name("${example.name}")
///             .vpn_authentication_types(vec!["Certificate",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Point-to-Site VPN Gateway's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/pointToPointVpnGateway:PointToPointVpnGateway example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/p2sVpnGateways/gateway1
/// ```
///
pub mod point_to_point_vpn_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PointToPointVpnGatewayArgs {
        /// A `connection_configuration` block as defined below.
        #[builder(into)]
        pub connection_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::network::PointToPointVpnGatewayConnectionConfiguration,
            >,
        >,
        /// A list of IP Addresses of DNS Servers for the Point-to-Site VPN Gateway.
        #[builder(into, default)]
        pub dns_servers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Point-to-Site VPN Gateway. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the Point-to-Site VPN Gateway. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Is the Routing Preference for the Public IP Interface of the VPN Gateway enabled? Defaults to `false`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub routing_preference_internet_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The [Scale Unit](https://docs.microsoft.com/azure/virtual-wan/virtual-wan-faq#what-is-a-virtual-wan-gateway-scale-unit) for this Point-to-Site VPN Gateway.
        #[builder(into)]
        pub scale_unit: pulumi_wasm_rust::Output<i32>,
        /// A mapping of tags to assign to the Point-to-Site VPN Gateway.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Virtual Hub where this Point-to-Site VPN Gateway should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_hub_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the VPN Server Configuration which this Point-to-Site VPN Gateway should use. Changing this forces a new resource to be created.
        #[builder(into)]
        pub vpn_server_configuration_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PointToPointVpnGatewayResult {
        /// A `connection_configuration` block as defined below.
        pub connection_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::network::PointToPointVpnGatewayConnectionConfiguration,
            >,
        >,
        /// A list of IP Addresses of DNS Servers for the Point-to-Site VPN Gateway.
        pub dns_servers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Point-to-Site VPN Gateway. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the Point-to-Site VPN Gateway. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Is the Routing Preference for the Public IP Interface of the VPN Gateway enabled? Defaults to `false`. Changing this forces a new resource to be created.
        pub routing_preference_internet_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The [Scale Unit](https://docs.microsoft.com/azure/virtual-wan/virtual-wan-faq#what-is-a-virtual-wan-gateway-scale-unit) for this Point-to-Site VPN Gateway.
        pub scale_unit: pulumi_wasm_rust::Output<i32>,
        /// A mapping of tags to assign to the Point-to-Site VPN Gateway.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Virtual Hub where this Point-to-Site VPN Gateway should exist. Changing this forces a new resource to be created.
        pub virtual_hub_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the VPN Server Configuration which this Point-to-Site VPN Gateway should use. Changing this forces a new resource to be created.
        pub vpn_server_configuration_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: PointToPointVpnGatewayArgs,
    ) -> PointToPointVpnGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let connection_configurations_binding = args
            .connection_configurations
            .get_inner();
        let dns_servers_binding = args.dns_servers.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let routing_preference_internet_enabled_binding = args
            .routing_preference_internet_enabled
            .get_inner();
        let scale_unit_binding = args.scale_unit.get_inner();
        let tags_binding = args.tags.get_inner();
        let virtual_hub_id_binding = args.virtual_hub_id.get_inner();
        let vpn_server_configuration_id_binding = args
            .vpn_server_configuration_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/pointToPointVpnGateway:PointToPointVpnGateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "connectionConfigurations".into(),
                    value: &connection_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "dnsServers".into(),
                    value: &dns_servers_binding,
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
                    name: "routingPreferenceInternetEnabled".into(),
                    value: &routing_preference_internet_enabled_binding,
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
                register_interface::ObjectField {
                    name: "vpnServerConfigurationId".into(),
                    value: &vpn_server_configuration_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "connectionConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "dnsServers".into(),
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
                    name: "routingPreferenceInternetEnabled".into(),
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
                register_interface::ResultField {
                    name: "vpnServerConfigurationId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PointToPointVpnGatewayResult {
            connection_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionConfigurations").unwrap(),
            ),
            dns_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsServers").unwrap(),
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
            routing_preference_internet_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routingPreferenceInternetEnabled").unwrap(),
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
            vpn_server_configuration_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpnServerConfigurationId").unwrap(),
            ),
        }
    }
}
