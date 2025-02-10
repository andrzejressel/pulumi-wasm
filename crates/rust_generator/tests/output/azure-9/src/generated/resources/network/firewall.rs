/// Manages an Azure Firewall.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod firewall {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallArgs {
        /// Whether DNS proxy is enabled. It will forward DNS requests to the DNS servers when set to `true`. It will be set to `true` if `dns_servers` provided with a not empty list.
        #[builder(into, default)]
        pub dns_proxy_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A list of DNS servers that the Azure Firewall will direct DNS traffic to the for name resolution.
        #[builder(into, default)]
        pub dns_servers: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ID of the Firewall Policy applied to this Firewall.
        #[builder(into, default)]
        pub firewall_policy_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `ip_configuration` block as documented below.
        #[builder(into, default)]
        pub ip_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::network::FirewallIpConfiguration>>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `management_ip_configuration` block as documented below, which allows force-tunnelling of traffic to be performed by the firewall. Adding or removing this block or changing the `subnet_id` in an existing block forces a new resource to be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub management_ip_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::FirewallManagementIpConfiguration>,
        >,
        /// Specifies the name of the Firewall. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of SNAT private CIDR IP ranges, or the special string `IANAPrivateRanges`, which indicates Azure Firewall does not SNAT when the destination IP address is a private range per IANA RFC 1918.
        #[builder(into, default)]
        pub private_ip_ranges: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name of the resource group in which to create the resource. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// SKU name of the Firewall. Possible values are `AZFW_Hub` and `AZFW_VNet`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// SKU tier of the Firewall. Possible values are `Premium`, `Standard` and `Basic`.
        #[builder(into)]
        pub sku_tier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The operation mode for threat intelligence-based filtering. Possible values are: `Off`, `Alert` and `Deny`. Defaults to `Alert`.
        #[builder(into, default)]
        pub threat_intel_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `virtual_hub` block as documented below.
        #[builder(into, default)]
        pub virtual_hub: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::FirewallVirtualHub>,
        >,
        /// Specifies a list of Availability Zones in which this Azure Firewall should be located. Changing this forces a new Azure Firewall to be created.
        ///
        /// > **Please Note**: Availability Zones are [only supported in several regions at this time](https://docs.microsoft.com/azure/availability-zones/az-overview).
        #[builder(into, default)]
        pub zones: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct FirewallResult {
        /// Whether DNS proxy is enabled. It will forward DNS requests to the DNS servers when set to `true`. It will be set to `true` if `dns_servers` provided with a not empty list.
        pub dns_proxy_enabled: pulumi_gestalt_rust::Output<bool>,
        /// A list of DNS servers that the Azure Firewall will direct DNS traffic to the for name resolution.
        pub dns_servers: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The ID of the Firewall Policy applied to this Firewall.
        pub firewall_policy_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `ip_configuration` block as documented below.
        pub ip_configurations: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::network::FirewallIpConfiguration>>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `management_ip_configuration` block as documented below, which allows force-tunnelling of traffic to be performed by the firewall. Adding or removing this block or changing the `subnet_id` in an existing block forces a new resource to be created. Changing this forces a new resource to be created.
        pub management_ip_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::network::FirewallManagementIpConfiguration>,
        >,
        /// Specifies the name of the Firewall. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of SNAT private CIDR IP ranges, or the special string `IANAPrivateRanges`, which indicates Azure Firewall does not SNAT when the destination IP address is a private range per IANA RFC 1918.
        pub private_ip_ranges: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The name of the resource group in which to create the resource. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// SKU name of the Firewall. Possible values are `AZFW_Hub` and `AZFW_VNet`. Changing this forces a new resource to be created.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// SKU tier of the Firewall. Possible values are `Premium`, `Standard` and `Basic`.
        pub sku_tier: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The operation mode for threat intelligence-based filtering. Possible values are: `Off`, `Alert` and `Deny`. Defaults to `Alert`.
        pub threat_intel_mode: pulumi_gestalt_rust::Output<String>,
        /// A `virtual_hub` block as documented below.
        pub virtual_hub: pulumi_gestalt_rust::Output<
            Option<super::super::types::network::FirewallVirtualHub>,
        >,
        /// Specifies a list of Availability Zones in which this Azure Firewall should be located. Changing this forces a new Azure Firewall to be created.
        ///
        /// > **Please Note**: Availability Zones are [only supported in several regions at this time](https://docs.microsoft.com/azure/availability-zones/az-overview).
        pub zones: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FirewallArgs,
    ) -> FirewallResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dns_proxy_enabled_binding = args.dns_proxy_enabled.get_output(context);
        let dns_servers_binding = args.dns_servers.get_output(context);
        let firewall_policy_id_binding = args.firewall_policy_id.get_output(context);
        let ip_configurations_binding = args.ip_configurations.get_output(context);
        let location_binding = args.location.get_output(context);
        let management_ip_configuration_binding = args
            .management_ip_configuration
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let private_ip_ranges_binding = args.private_ip_ranges.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_name_binding = args.sku_name.get_output(context);
        let sku_tier_binding = args.sku_tier.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let threat_intel_mode_binding = args.threat_intel_mode.get_output(context);
        let virtual_hub_binding = args.virtual_hub.get_output(context);
        let zones_binding = args.zones.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/firewall:Firewall".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsProxyEnabled".into(),
                    value: dns_proxy_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsServers".into(),
                    value: dns_servers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firewallPolicyId".into(),
                    value: firewall_policy_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipConfigurations".into(),
                    value: ip_configurations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managementIpConfiguration".into(),
                    value: management_ip_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateIpRanges".into(),
                    value: private_ip_ranges_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuName".into(),
                    value: sku_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuTier".into(),
                    value: sku_tier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "threatIntelMode".into(),
                    value: threat_intel_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualHub".into(),
                    value: virtual_hub_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zones".into(),
                    value: zones_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FirewallResult {
            dns_proxy_enabled: o.get_field("dnsProxyEnabled"),
            dns_servers: o.get_field("dnsServers"),
            firewall_policy_id: o.get_field("firewallPolicyId"),
            ip_configurations: o.get_field("ipConfigurations"),
            location: o.get_field("location"),
            management_ip_configuration: o.get_field("managementIpConfiguration"),
            name: o.get_field("name"),
            private_ip_ranges: o.get_field("privateIpRanges"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku_name: o.get_field("skuName"),
            sku_tier: o.get_field("skuTier"),
            tags: o.get_field("tags"),
            threat_intel_mode: o.get_field("threatIntelMode"),
            virtual_hub: o.get_field("virtualHub"),
            zones: o.get_field("zones"),
        }
    }
}
