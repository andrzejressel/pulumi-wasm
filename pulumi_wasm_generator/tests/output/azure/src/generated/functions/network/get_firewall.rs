pub mod get_firewall {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFirewallArgs {
        /// Whether DNS proxy is enabled. It will forward DNS requests to the DNS servers when it is `true`.
        #[builder(into, default)]
        pub dns_proxy_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Azure Firewall.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the Azure Firewall exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetFirewallResult {
        /// Whether DNS proxy is enabled. It will forward DNS requests to the DNS servers when it is `true`.
        pub dns_proxy_enabled: pulumi_wasm_rust::Output<bool>,
        /// The list of DNS servers that the Azure Firewall will direct DNS traffic to for name resolution.
        pub dns_servers: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the Firewall Policy applied to the Azure Firewall.
        pub firewall_policy_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A `ip_configuration` block as defined below.
        pub ip_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetFirewallIpConfiguration>,
        >,
        /// The Azure location where the Azure Firewall exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `management_ip_configuration` block as defined below, which allows force-tunnelling of traffic to be performed by the firewall.
        pub management_ip_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetFirewallManagementIpConfiguration,
            >,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The SKU name of the Azure Firewall.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// The SKU tier of the Azure Firewall.
        pub sku_tier: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the Azure Firewall.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The operation mode for threat intelligence-based filtering.
        pub threat_intel_mode: pulumi_wasm_rust::Output<String>,
        /// A `virtual_hub` block as defined below.
        pub virtual_hubs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetFirewallVirtualHub>,
        >,
        /// A list of Availability Zones in which this Azure Firewall is located.
        pub zones: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetFirewallArgs) -> GetFirewallResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dns_proxy_enabled_binding = args.dns_proxy_enabled.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getFirewall:getFirewall".into(),
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
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "managementIpConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
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
                    name: "virtualHubs".into(),
                },
                register_interface::ResultField {
                    name: "zones".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetFirewallResult {
            dns_proxy_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsProxyEnabled").unwrap(),
            ),
            dns_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsServers").unwrap(),
            ),
            firewall_policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firewallPolicyId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ip_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipConfigurations").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            management_ip_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managementIpConfigurations").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
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
            virtual_hubs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualHubs").unwrap(),
            ),
            zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zones").unwrap(),
            ),
        }
    }
}