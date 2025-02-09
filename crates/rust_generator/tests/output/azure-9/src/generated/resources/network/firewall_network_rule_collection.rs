/// Manages a Network Rule Collection within an Azure Firewall.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod firewall_network_rule_collection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallNetworkRuleCollectionArgs {
        /// Specifies the action the rule will apply to matching traffic. Possible values are `Allow` and `Deny`.
        #[builder(into)]
        pub action: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Firewall in which the Network Rule Collection should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub azure_firewall_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Network Rule Collection which must be unique within the Firewall. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the priority of the rule collection. Possible values are between `100` - `65000`.
        #[builder(into)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Specifies the name of the Resource Group in which the Firewall exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `rule` blocks as defined below.
        #[builder(into)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::network::FirewallNetworkRuleCollectionRule>,
        >,
    }
    #[allow(dead_code)]
    pub struct FirewallNetworkRuleCollectionResult {
        /// Specifies the action the rule will apply to matching traffic. Possible values are `Allow` and `Deny`.
        pub action: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Firewall in which the Network Rule Collection should be created. Changing this forces a new resource to be created.
        pub azure_firewall_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Network Rule Collection which must be unique within the Firewall. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the priority of the rule collection. Possible values are between `100` - `65000`.
        pub priority: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the name of the Resource Group in which the Firewall exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// One or more `rule` blocks as defined below.
        pub rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::network::FirewallNetworkRuleCollectionRule>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FirewallNetworkRuleCollectionArgs,
    ) -> FirewallNetworkRuleCollectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let action_binding = args.action.get_output(context);
        let azure_firewall_name_binding = args.azure_firewall_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let rules_binding = args.rules.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/firewallNetworkRuleCollection:FirewallNetworkRuleCollection"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "action".into(),
                    value: action_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "azureFirewallName".into(),
                    value: azure_firewall_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: priority_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rules".into(),
                    value: rules_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FirewallNetworkRuleCollectionResult {
            action: o.get_field("action"),
            azure_firewall_name: o.get_field("azureFirewallName"),
            name: o.get_field("name"),
            priority: o.get_field("priority"),
            resource_group_name: o.get_field("resourceGroupName"),
            rules: o.get_field("rules"),
        }
    }
}
