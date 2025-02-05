/// Manages a Palo Alto Next Generation Firewall Deployed in a Virtual Network and configured via a Local Rulestack.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resource-group
///       location: westeurope
///   examplePublicIp:
///     type: azure:network:PublicIp
///     name: example
///     properties:
///       name: example-public-ip
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       allocationMethod: Static
///       sku: Standard
///   exampleNetworkSecurityGroup:
///     type: azure:network:NetworkSecurityGroup
///     name: example
///     properties:
///       name: example-nsg
///       location: ${test.location}
///       resourceGroupName: ${test.name}
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tags:
///         environment: Production
///   trust:
///     type: azure:network:Subnet
///     properties:
///       name: example-trust-subnet
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.1.0/24
///       delegations:
///         - name: trusted
///           serviceDelegation:
///             name: PaloAltoNetworks.Cloudngfw/firewalls
///             actions:
///               - Microsoft.Network/virtualNetworks/subnets/join/action
///   trustSubnetNetworkSecurityGroupAssociation:
///     type: azure:network:SubnetNetworkSecurityGroupAssociation
///     name: trust
///     properties:
///       subnetId: ${trust.id}
///       networkSecurityGroupId: ${exampleNetworkSecurityGroup.id}
///   untrust:
///     type: azure:network:Subnet
///     properties:
///       name: example-untrust-subnet
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///       delegations:
///         - name: untrusted
///           serviceDelegation:
///             name: PaloAltoNetworks.Cloudngfw/firewalls
///             actions:
///               - Microsoft.Network/virtualNetworks/subnets/join/action
///   untrustSubnetNetworkSecurityGroupAssociation:
///     type: azure:network:SubnetNetworkSecurityGroupAssociation
///     name: untrust
///     properties:
///       subnetId: ${untrust.id}
///       networkSecurityGroupId: ${exampleNetworkSecurityGroup.id}
///   exampleLocalRulestack:
///     type: azure:paloalto:LocalRulestack
///     name: example
///     properties:
///       name: example-rulestack
///       resourceGroupName: ${example.name}
///       location: ${example.locatio}
///   exampleLocalRulestackRule:
///     type: azure:paloalto:LocalRulestackRule
///     name: example
///     properties:
///       name: example-rulestack-rule
///       rulestackId: ${exampleLocalRulestack.id}
///       priority: 1001
///       action: Allow
///       applications:
///         - any
///       destination:
///         cidrs:
///           - any
///       source:
///         cidrs:
///           - any
///   exampleNextGenerationFirewallVirtualNetworkLocalRulestack:
///     type: azure:paloalto:NextGenerationFirewallVirtualNetworkLocalRulestack
///     name: example
///     properties:
///       name: example-ngfwvn
///       resourceGroupName: ${example.name}
///       rulestackId: ${exampleLocalRulestack.id}
///       networkProfile:
///         publicIpAddressIds:
///           - ${examplePublicIp.id}
///         vnetConfiguration:
///           virtualNetworkId: ${exampleVirtualNetwork.id}
///           trustedSubnetId: ${trust.id}
///           untrustedSubnetId: ${untrust.id}
/// ```
///
/// ## Import
///
/// Palo Alto Next Generation Firewall Virtual Network Local Rulestacks can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:paloalto/nextGenerationFirewallVirtualNetworkLocalRulestack:NextGenerationFirewallVirtualNetworkLocalRulestack example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/PaloAltoNetworks.Cloudngfw/firewalls/myVNetRulestackFW
/// ```
///
pub mod next_generation_firewall_virtual_network_local_rulestack {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NextGenerationFirewallVirtualNetworkLocalRulestackArgs {
        /// One or more `destination_nat` blocks as defined below.
        #[builder(into, default)]
        pub destination_nats: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::paloalto::NextGenerationFirewallVirtualNetworkLocalRulestackDestinationNat,
                >,
            >,
        >,
        /// A `dns_settings` block as defined below.
        #[builder(into, default)]
        pub dns_settings: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::paloalto::NextGenerationFirewallVirtualNetworkLocalRulestackDnsSettings,
            >,
        >,
        /// The name which should be used for this Palo Alto Next Generation Firewall Virtual Network Local Rulestack. Changing this forces a new Palo Alto Next Generation Firewall Virtual Network Local Rulestack to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `network_profile` block as defined below.
        #[builder(into)]
        pub network_profile: pulumi_wasm_rust::InputOrOutput<
            super::super::types::paloalto::NextGenerationFirewallVirtualNetworkLocalRulestackNetworkProfile,
        >,
        /// The name of the Resource Group where the Palo Alto Next Generation Firewall Virtual Network Local Rulestack should exist. Changing this forces a new Palo Alto Next Generation Firewall Virtual Network Local Rulestack to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the Local Rulestack which will be used to configure this Firewall Resource.
        #[builder(into)]
        pub rulestack_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Palo Alto Next Generation Firewall Virtual Network Local Rulestack.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NextGenerationFirewallVirtualNetworkLocalRulestackResult {
        /// One or more `destination_nat` blocks as defined below.
        pub destination_nats: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::paloalto::NextGenerationFirewallVirtualNetworkLocalRulestackDestinationNat,
                >,
            >,
        >,
        /// A `dns_settings` block as defined below.
        pub dns_settings: pulumi_wasm_rust::Output<
            Option<
                super::super::types::paloalto::NextGenerationFirewallVirtualNetworkLocalRulestackDnsSettings,
            >,
        >,
        /// The name which should be used for this Palo Alto Next Generation Firewall Virtual Network Local Rulestack. Changing this forces a new Palo Alto Next Generation Firewall Virtual Network Local Rulestack to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `network_profile` block as defined below.
        pub network_profile: pulumi_wasm_rust::Output<
            super::super::types::paloalto::NextGenerationFirewallVirtualNetworkLocalRulestackNetworkProfile,
        >,
        /// The name of the Resource Group where the Palo Alto Next Generation Firewall Virtual Network Local Rulestack should exist. Changing this forces a new Palo Alto Next Generation Firewall Virtual Network Local Rulestack to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Local Rulestack which will be used to configure this Firewall Resource.
        pub rulestack_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Palo Alto Next Generation Firewall Virtual Network Local Rulestack.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: NextGenerationFirewallVirtualNetworkLocalRulestackArgs,
    ) -> NextGenerationFirewallVirtualNetworkLocalRulestackResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let destination_nats_binding = args
            .destination_nats
            .get_output(context)
            .get_inner();
        let dns_settings_binding = args.dns_settings.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_profile_binding = args
            .network_profile
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let rulestack_id_binding = args.rulestack_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:paloalto/nextGenerationFirewallVirtualNetworkLocalRulestack:NextGenerationFirewallVirtualNetworkLocalRulestack"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "destinationNats".into(),
                    value: &destination_nats_binding,
                },
                register_interface::ObjectField {
                    name: "dnsSettings".into(),
                    value: &dns_settings_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkProfile".into(),
                    value: &network_profile_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "rulestackId".into(),
                    value: &rulestack_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        NextGenerationFirewallVirtualNetworkLocalRulestackResult {
            destination_nats: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("destinationNats"),
            ),
            dns_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dnsSettings"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            network_profile: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("networkProfile"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            rulestack_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("rulestackId"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
