/// Manages a Network Rule Collection within an Azure Firewall.
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
///     let exampleFirewallNetworkRuleCollection = firewall_network_rule_collection::create(
///         "exampleFirewallNetworkRuleCollection",
///         FirewallNetworkRuleCollectionArgs::builder()
///             .action("Allow")
///             .azure_firewall_name("${exampleFirewall.name}")
///             .name("testcollection")
///             .priority(100)
///             .resource_group_name("${example.name}")
///             .rules(
///                 vec![
///                     FirewallNetworkRuleCollectionRule::builder()
///                     .destinationAddresses(vec!["8.8.8.8", "8.8.4.4",])
///                     .destinationPorts(vec!["53",]).name("testrule").protocols(vec!["TCP",
///                     "UDP",]).sourceAddresses(vec!["10.0.0.0/16",]).build_struct(),
///                 ],
///             )
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
/// Azure Firewall Network Rule Collections can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/firewallNetworkRuleCollection:FirewallNetworkRuleCollection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/azureFirewalls/myfirewall/networkRuleCollections/mycollection
/// ```
///
pub mod firewall_network_rule_collection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallNetworkRuleCollectionArgs {
        /// Specifies the action the rule will apply to matching traffic. Possible values are `Allow` and `Deny`.
        #[builder(into)]
        pub action: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Firewall in which the Network Rule Collection should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub azure_firewall_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Network Rule Collection which must be unique within the Firewall. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the priority of the rule collection. Possible values are between `100` - `65000`.
        #[builder(into)]
        pub priority: pulumi_wasm_rust::Output<i32>,
        /// Specifies the name of the Resource Group in which the Firewall exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// One or more `rule` blocks as defined below.
        #[builder(into)]
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::FirewallNetworkRuleCollectionRule>,
        >,
    }
    #[allow(dead_code)]
    pub struct FirewallNetworkRuleCollectionResult {
        /// Specifies the action the rule will apply to matching traffic. Possible values are `Allow` and `Deny`.
        pub action: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Firewall in which the Network Rule Collection should be created. Changing this forces a new resource to be created.
        pub azure_firewall_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Network Rule Collection which must be unique within the Firewall. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the priority of the rule collection. Possible values are between `100` - `65000`.
        pub priority: pulumi_wasm_rust::Output<i32>,
        /// Specifies the name of the Resource Group in which the Firewall exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// One or more `rule` blocks as defined below.
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::FirewallNetworkRuleCollectionRule>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: FirewallNetworkRuleCollectionArgs,
    ) -> FirewallNetworkRuleCollectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_inner();
        let azure_firewall_name_binding = args.azure_firewall_name.get_inner();
        let name_binding = args.name.get_inner();
        let priority_binding = args.priority.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let rules_binding = args.rules.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/firewallNetworkRuleCollection:FirewallNetworkRuleCollection"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "action".into(),
                    value: &action_binding,
                },
                register_interface::ObjectField {
                    name: "azureFirewallName".into(),
                    value: &azure_firewall_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "action".into(),
                },
                register_interface::ResultField {
                    name: "azureFirewallName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "priority".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "rules".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FirewallNetworkRuleCollectionResult {
            action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("action").unwrap(),
            ),
            azure_firewall_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("azureFirewallName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priority").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rules").unwrap(),
            ),
        }
    }
}