#[allow(clippy::doc_lazy_continuation)]
pub mod get_firewall {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFirewallArgs {
        /// Whether DNS proxy is enabled. It will forward DNS requests to the DNS servers when it is `true`.
        #[builder(into, default)]
        pub dns_proxy_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the Azure Firewall.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the Azure Firewall exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFirewallResult {
        /// Whether DNS proxy is enabled. It will forward DNS requests to the DNS servers when it is `true`.
        pub dns_proxy_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The list of DNS servers that the Azure Firewall will direct DNS traffic to for name resolution.
        pub dns_servers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ID of the Firewall Policy applied to the Azure Firewall.
        pub firewall_policy_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A `ip_configuration` block as defined below.
        pub ip_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetFirewallIpConfiguration>,
        >,
        /// The Azure location where the Azure Firewall exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `management_ip_configuration` block as defined below, which allows force-tunnelling of traffic to be performed by the firewall.
        pub management_ip_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetFirewallManagementIpConfiguration,
            >,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The SKU name of the Azure Firewall.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// The SKU tier of the Azure Firewall.
        pub sku_tier: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the Azure Firewall.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The operation mode for threat intelligence-based filtering.
        pub threat_intel_mode: pulumi_gestalt_rust::Output<String>,
        /// A `virtual_hub` block as defined below.
        pub virtual_hubs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetFirewallVirtualHub>,
        >,
        /// A list of Availability Zones in which this Azure Firewall is located.
        pub zones: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetFirewallArgs,
    ) -> GetFirewallResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let dns_proxy_enabled_binding = args
            .dns_proxy_enabled
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getFirewall:getFirewall".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dnsProxyEnabled".into(),
                    value: &dns_proxy_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetFirewallResult {
            dns_proxy_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsProxyEnabled"),
            ),
            dns_servers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsServers"),
            ),
            firewall_policy_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("firewallPolicyId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ip_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipConfigurations"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            management_ip_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managementIpConfigurations"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
            sku_tier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skuTier"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            threat_intel_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("threatIntelMode"),
            ),
            virtual_hubs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("virtualHubs"),
            ),
            zones: pulumi_gestalt_rust::__private::into_domain(o.extract_field("zones")),
        }
    }
}
