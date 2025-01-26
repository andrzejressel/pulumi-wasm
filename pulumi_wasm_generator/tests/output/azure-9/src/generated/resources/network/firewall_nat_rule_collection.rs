/// Manages a NAT Rule Collection within an Azure Firewall.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: testvnet
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: AzureFirewallSubnet
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.1.0/24
///   examplePublicIp:
///     type: azure:network:PublicIp
///     name: example
///     properties:
///       name: testpip
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       allocationMethod: Static
///       sku: Standard
///   exampleFirewall:
///     type: azure:network:Firewall
///     name: example
///     properties:
///       name: testfirewall
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: AZFW_VNet
///       skuTier: Standard
///       ipConfigurations:
///         - name: configuration
///           subnetId: ${exampleSubnet.id}
///           publicIpAddressId: ${examplePublicIp.id}
///   exampleFirewallNatRuleCollection:
///     type: azure:network:FirewallNatRuleCollection
///     name: example
///     properties:
///       name: testcollection
///       azureFirewallName: ${exampleFirewall.name}
///       resourceGroupName: ${example.name}
///       priority: 100
///       action: Dnat
///       rules:
///         - name: testrule
///           sourceAddresses:
///             - 10.0.0.0/16
///           destinationPorts:
///             - '53'
///           destinationAddresses:
///             - ${examplePublicIp.ipAddress}
///           translatedPort: 53
///           translatedAddress: 8.8.8.8
///           protocols:
///             - TCP
///             - UDP
/// ```
///
/// ## Import
///
/// Azure Firewall NAT Rule Collections can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/firewallNatRuleCollection:FirewallNatRuleCollection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/azureFirewalls/myfirewall/natRuleCollections/mycollection
/// ```
///
pub mod firewall_nat_rule_collection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallNatRuleCollectionArgs {
        /// Specifies the action the rule will apply to matching traffic. Possible values are `Dnat` and `Snat`.
        #[builder(into)]
        pub action: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the Firewall in which the NAT Rule Collection should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub azure_firewall_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the NAT Rule Collection which must be unique within the Firewall. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the priority of the rule collection. Possible values are between `100` - `65000`.
        #[builder(into)]
        pub priority: pulumi_wasm_rust::InputOrOutput<i32>,
        /// Specifies the name of the Resource Group in which the Firewall exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// One or more `rule` blocks as defined below.
        #[builder(into)]
        pub rules: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::network::FirewallNatRuleCollectionRule>,
        >,
    }
    #[allow(dead_code)]
    pub struct FirewallNatRuleCollectionResult {
        /// Specifies the action the rule will apply to matching traffic. Possible values are `Dnat` and `Snat`.
        pub action: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Firewall in which the NAT Rule Collection should be created. Changing this forces a new resource to be created.
        pub azure_firewall_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the NAT Rule Collection which must be unique within the Firewall. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the priority of the rule collection. Possible values are between `100` - `65000`.
        pub priority: pulumi_wasm_rust::Output<i32>,
        /// Specifies the name of the Resource Group in which the Firewall exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// One or more `rule` blocks as defined below.
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::FirewallNatRuleCollectionRule>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FirewallNatRuleCollectionArgs,
    ) -> FirewallNatRuleCollectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_output(context).get_inner();
        let azure_firewall_name_binding = args
            .azure_firewall_name
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let priority_binding = args.priority.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let rules_binding = args.rules.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/firewallNatRuleCollection:FirewallNatRuleCollection"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        FirewallNatRuleCollectionResult {
            action: pulumi_wasm_rust::__private::into_domain(o.extract_field("action")),
            azure_firewall_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("azureFirewallName"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            priority: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("priority"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(o.extract_field("rules")),
        }
    }
}
