/// Manages an Azure Firewall.
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
///     let exampleFirewall = firewall::create(
///         "exampleFirewall",
///         FirewallArgs::builder()
///             .ip_configurations(
///                 vec![
///                     FirewallIpConfiguration::builder().name("configuration")
///                     .publicIpAddressId("${examplePublicIp.id}")
///                     .subnetId("${exampleSubnet.id}").build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("testfirewall")
///             .resource_group_name("${example.name}")
///             .sku_name("AZFW_VNet")
///             .sku_tier("Standard")
///             .build_struct(),
///     );
///     let examplePublicIp = public_ip::create(
///         "examplePublicIp",
///         PublicIpArgs::builder()
///             .allocation_method("Static")
///             .location("${example.location}")
///             .name("testpip")
///             .resource_group_name("${example.name}")
///             .sku("Standard")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.1.0/24",])
///             .name("AzureFirewallSubnet")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("testvnet")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Azure Firewalls can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/firewall:Firewall example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/azureFirewalls/testfirewall
/// ```
///
pub mod firewall {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallArgs {
        /// Whether DNS proxy is enabled. It will forward DNS requests to the DNS servers when set to `true`. It will be set to `true` if `dns_servers` provided with a not empty list.
        #[builder(into, default)]
        pub dns_proxy_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of DNS servers that the Azure Firewall will direct DNS traffic to the for name resolution.
        #[builder(into, default)]
        pub dns_servers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ID of the Firewall Policy applied to this Firewall.
        #[builder(into, default)]
        pub firewall_policy_id: pulumi_wasm_rust::Output<Option<String>>,
        /// An `ip_configuration` block as documented below.
        #[builder(into, default)]
        pub ip_configurations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::FirewallIpConfiguration>>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// A `management_ip_configuration` block as documented below, which allows force-tunnelling of traffic to be performed by the firewall. Adding or removing this block or changing the `subnet_id` in an existing block forces a new resource to be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub management_ip_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::network::FirewallManagementIpConfiguration>,
        >,
        /// Specifies the name of the Firewall. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of SNAT private CIDR IP ranges, or the special string `IANAPrivateRanges`, which indicates Azure Firewall does not SNAT when the destination IP address is a private range per IANA RFC 1918.
        #[builder(into, default)]
        pub private_ip_ranges: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name of the resource group in which to create the resource. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// SKU name of the Firewall. Possible values are `AZFW_Hub` and `AZFW_VNet`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// SKU tier of the Firewall. Possible values are `Premium`, `Standard` and `Basic`.
        #[builder(into)]
        pub sku_tier: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The operation mode for threat intelligence-based filtering. Possible values are: `Off`, `Alert` and `Deny`. Defaults to `Alert`.
        #[builder(into, default)]
        pub threat_intel_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// A `virtual_hub` block as documented below.
        #[builder(into, default)]
        pub virtual_hub: pulumi_wasm_rust::Output<
            Option<super::super::types::network::FirewallVirtualHub>,
        >,
        /// Specifies a list of Availability Zones in which this Azure Firewall should be located. Changing this forces a new Azure Firewall to be created.
        ///
        /// > **Please Note**: Availability Zones are [only supported in several regions at this time](https://docs.microsoft.com/azure/availability-zones/az-overview).
        #[builder(into, default)]
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct FirewallResult {
        /// Whether DNS proxy is enabled. It will forward DNS requests to the DNS servers when set to `true`. It will be set to `true` if `dns_servers` provided with a not empty list.
        pub dns_proxy_enabled: pulumi_wasm_rust::Output<bool>,
        /// A list of DNS servers that the Azure Firewall will direct DNS traffic to the for name resolution.
        pub dns_servers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ID of the Firewall Policy applied to this Firewall.
        pub firewall_policy_id: pulumi_wasm_rust::Output<Option<String>>,
        /// An `ip_configuration` block as documented below.
        pub ip_configurations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::FirewallIpConfiguration>>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `management_ip_configuration` block as documented below, which allows force-tunnelling of traffic to be performed by the firewall. Adding or removing this block or changing the `subnet_id` in an existing block forces a new resource to be created. Changing this forces a new resource to be created.
        pub management_ip_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::network::FirewallManagementIpConfiguration>,
        >,
        /// Specifies the name of the Firewall. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of SNAT private CIDR IP ranges, or the special string `IANAPrivateRanges`, which indicates Azure Firewall does not SNAT when the destination IP address is a private range per IANA RFC 1918.
        pub private_ip_ranges: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name of the resource group in which to create the resource. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// SKU name of the Firewall. Possible values are `AZFW_Hub` and `AZFW_VNet`. Changing this forces a new resource to be created.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// SKU tier of the Firewall. Possible values are `Premium`, `Standard` and `Basic`.
        pub sku_tier: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The operation mode for threat intelligence-based filtering. Possible values are: `Off`, `Alert` and `Deny`. Defaults to `Alert`.
        pub threat_intel_mode: pulumi_wasm_rust::Output<String>,
        /// A `virtual_hub` block as documented below.
        pub virtual_hub: pulumi_wasm_rust::Output<
            Option<super::super::types::network::FirewallVirtualHub>,
        >,
        /// Specifies a list of Availability Zones in which this Azure Firewall should be located. Changing this forces a new Azure Firewall to be created.
        ///
        /// > **Please Note**: Availability Zones are [only supported in several regions at this time](https://docs.microsoft.com/azure/availability-zones/az-overview).
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FirewallArgs) -> FirewallResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dns_proxy_enabled_binding = args.dns_proxy_enabled.get_inner();
        let dns_servers_binding = args.dns_servers.get_inner();
        let firewall_policy_id_binding = args.firewall_policy_id.get_inner();
        let ip_configurations_binding = args.ip_configurations.get_inner();
        let location_binding = args.location.get_inner();
        let management_ip_configuration_binding = args
            .management_ip_configuration
            .get_inner();
        let name_binding = args.name.get_inner();
        let private_ip_ranges_binding = args.private_ip_ranges.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sku_name_binding = args.sku_name.get_inner();
        let sku_tier_binding = args.sku_tier.get_inner();
        let tags_binding = args.tags.get_inner();
        let threat_intel_mode_binding = args.threat_intel_mode.get_inner();
        let virtual_hub_binding = args.virtual_hub.get_inner();
        let zones_binding = args.zones.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/firewall:Firewall".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dnsProxyEnabled".into(),
                    value: &dns_proxy_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "dnsServers".into(),
                    value: &dns_servers_binding,
                },
                register_interface::ObjectField {
                    name: "firewallPolicyId".into(),
                    value: &firewall_policy_id_binding,
                },
                register_interface::ObjectField {
                    name: "ipConfigurations".into(),
                    value: &ip_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "managementIpConfiguration".into(),
                    value: &management_ip_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "privateIpRanges".into(),
                    value: &private_ip_ranges_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "skuTier".into(),
                    value: &sku_tier_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "threatIntelMode".into(),
                    value: &threat_intel_mode_binding,
                },
                register_interface::ObjectField {
                    name: "virtualHub".into(),
                    value: &virtual_hub_binding,
                },
                register_interface::ObjectField {
                    name: "zones".into(),
                    value: &zones_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dnsProxyEnabled".into(),
                },
                register_interface::ResultField {
                    name: "dnsServers".into(),
                },
                register_interface::ResultField {
                    name: "firewallPolicyId".into(),
                },
                register_interface::ResultField {
                    name: "ipConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "managementIpConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "privateIpRanges".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "skuTier".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "threatIntelMode".into(),
                },
                register_interface::ResultField {
                    name: "virtualHub".into(),
                },
                register_interface::ResultField {
                    name: "zones".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FirewallResult {
            dns_proxy_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsProxyEnabled").unwrap(),
            ),
            dns_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsServers").unwrap(),
            ),
            firewall_policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firewallPolicyId").unwrap(),
            ),
            ip_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipConfigurations").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            management_ip_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managementIpConfiguration").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            private_ip_ranges: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateIpRanges").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            sku_tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuTier").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            threat_intel_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("threatIntelMode").unwrap(),
            ),
            virtual_hub: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualHub").unwrap(),
            ),
            zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zones").unwrap(),
            ),
        }
    }
}