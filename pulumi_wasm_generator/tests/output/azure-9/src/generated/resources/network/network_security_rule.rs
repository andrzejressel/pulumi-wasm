/// Manages a Network Security Rule.
///
/// > **NOTE on Network Security Groups and Network Security Rules:** This provider currently
/// provides both a standalone Network Security Rule resource, and allows for Network Security Rules to be defined in-line within the Network Security Group resource.
/// At this time you cannot use a Network Security Group with in-line Network Security Rules in conjunction with any Network Security Rule resources. Doing so will cause a conflict of rule settings and will overwrite rules.
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
///     let exampleNetworkSecurityGroup = network_security_group::create(
///         "exampleNetworkSecurityGroup",
///         NetworkSecurityGroupArgs::builder()
///             .location("${example.location}")
///             .name("acceptanceTestSecurityGroup1")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleNetworkSecurityRule = network_security_rule::create(
///         "exampleNetworkSecurityRule",
///         NetworkSecurityRuleArgs::builder()
///             .access("Allow")
///             .destination_address_prefix("*")
///             .destination_port_range("*")
///             .direction("Outbound")
///             .name("test123")
///             .network_security_group_name("${exampleNetworkSecurityGroup.name}")
///             .priority(100)
///             .protocol("Tcp")
///             .resource_group_name("${example.name}")
///             .source_address_prefix("*")
///             .source_port_range("*")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Network Security Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/networkSecurityRule:NetworkSecurityRule rule1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/networkSecurityGroups/mySecurityGroup/securityRules/rule1
/// ```
///
pub mod network_security_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkSecurityRuleArgs {
        /// Specifies whether network traffic is allowed or denied. Possible values are `Allow` and `Deny`.
        #[builder(into)]
        pub access: pulumi_wasm_rust::InputOrOutput<String>,
        /// A description for this rule. Restricted to 140 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// CIDR or destination IP range or * to match any IP. Tags such as `VirtualNetwork`, `AzureLoadBalancer` and `Internet` can also be used. Besides, it also supports all available Service Tags like ‘Sql.WestEurope‘, ‘Storage.EastUS‘, etc. You can list the available service tags with the CLI: ```shell az network list-service-tags --location westcentralus```. For further information please see [Azure CLI - az network list-service-tags](https://docs.microsoft.com/cli/azure/network?view=azure-cli-latest#az-network-list-service-tags). This is required if `destination_address_prefixes` is not specified.
        #[builder(into, default)]
        pub destination_address_prefix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// List of destination address prefixes. Tags may not be used. This is required if `destination_address_prefix` is not specified.
        #[builder(into, default)]
        pub destination_address_prefixes: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// A List of destination Application Security Group IDs
        #[builder(into, default)]
        pub destination_application_security_group_ids: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Destination Port or Range. Integer or range between `0` and `65535` or `*` to match any. This is required if `destination_port_ranges` is not specified.
        #[builder(into, default)]
        pub destination_port_range: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// List of destination ports or port ranges. This is required if `destination_port_range` is not specified.
        #[builder(into, default)]
        pub destination_port_ranges: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The direction specifies if rule will be evaluated on incoming or outgoing traffic. Possible values are `Inbound` and `Outbound`.
        #[builder(into)]
        pub direction: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the security rule. This needs to be unique across all Rules in the Network Security Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Network Security Group that we want to attach the rule to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub network_security_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the priority of the rule. The value can be between 100 and 4096. The priority number must be unique for each rule in the collection. The lower the priority number, the higher the priority of the rule.
        #[builder(into)]
        pub priority: pulumi_wasm_rust::InputOrOutput<i32>,
        /// Network protocol this rule applies to. Possible values include `Tcp`, `Udp`, `Icmp`, `Esp`, `Ah` or `*` (which matches all).
        #[builder(into)]
        pub protocol: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the resource group in which to create the Network Security Rule. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// CIDR or source IP range or * to match any IP. Tags such as `VirtualNetwork`, `AzureLoadBalancer` and `Internet` can also be used. This is required if `source_address_prefixes` is not specified.
        #[builder(into, default)]
        pub source_address_prefix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// List of source address prefixes. Tags may not be used. This is required if `source_address_prefix` is not specified.
        #[builder(into, default)]
        pub source_address_prefixes: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// A List of source Application Security Group IDs
        #[builder(into, default)]
        pub source_application_security_group_ids: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Source Port or Range. Integer or range between `0` and `65535` or `*` to match any. This is required if `source_port_ranges` is not specified.
        #[builder(into, default)]
        pub source_port_range: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// List of source ports or port ranges. This is required if `source_port_range` is not specified.
        #[builder(into, default)]
        pub source_port_ranges: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct NetworkSecurityRuleResult {
        /// Specifies whether network traffic is allowed or denied. Possible values are `Allow` and `Deny`.
        pub access: pulumi_wasm_rust::Output<String>,
        /// A description for this rule. Restricted to 140 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// CIDR or destination IP range or * to match any IP. Tags such as `VirtualNetwork`, `AzureLoadBalancer` and `Internet` can also be used. Besides, it also supports all available Service Tags like ‘Sql.WestEurope‘, ‘Storage.EastUS‘, etc. You can list the available service tags with the CLI: ```shell az network list-service-tags --location westcentralus```. For further information please see [Azure CLI - az network list-service-tags](https://docs.microsoft.com/cli/azure/network?view=azure-cli-latest#az-network-list-service-tags). This is required if `destination_address_prefixes` is not specified.
        pub destination_address_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// List of destination address prefixes. Tags may not be used. This is required if `destination_address_prefix` is not specified.
        pub destination_address_prefixes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A List of destination Application Security Group IDs
        pub destination_application_security_group_ids: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// Destination Port or Range. Integer or range between `0` and `65535` or `*` to match any. This is required if `destination_port_ranges` is not specified.
        pub destination_port_range: pulumi_wasm_rust::Output<Option<String>>,
        /// List of destination ports or port ranges. This is required if `destination_port_range` is not specified.
        pub destination_port_ranges: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The direction specifies if rule will be evaluated on incoming or outgoing traffic. Possible values are `Inbound` and `Outbound`.
        pub direction: pulumi_wasm_rust::Output<String>,
        /// The name of the security rule. This needs to be unique across all Rules in the Network Security Group. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Network Security Group that we want to attach the rule to. Changing this forces a new resource to be created.
        pub network_security_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the priority of the rule. The value can be between 100 and 4096. The priority number must be unique for each rule in the collection. The lower the priority number, the higher the priority of the rule.
        pub priority: pulumi_wasm_rust::Output<i32>,
        /// Network protocol this rule applies to. Possible values include `Tcp`, `Udp`, `Icmp`, `Esp`, `Ah` or `*` (which matches all).
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the Network Security Rule. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// CIDR or source IP range or * to match any IP. Tags such as `VirtualNetwork`, `AzureLoadBalancer` and `Internet` can also be used. This is required if `source_address_prefixes` is not specified.
        pub source_address_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// List of source address prefixes. Tags may not be used. This is required if `source_address_prefix` is not specified.
        pub source_address_prefixes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A List of source Application Security Group IDs
        pub source_application_security_group_ids: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// Source Port or Range. Integer or range between `0` and `65535` or `*` to match any. This is required if `source_port_ranges` is not specified.
        pub source_port_range: pulumi_wasm_rust::Output<Option<String>>,
        /// List of source ports or port ranges. This is required if `source_port_range` is not specified.
        pub source_port_ranges: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: NetworkSecurityRuleArgs,
    ) -> NetworkSecurityRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_binding = args.access.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let destination_address_prefix_binding = args
            .destination_address_prefix
            .get_output(context)
            .get_inner();
        let destination_address_prefixes_binding = args
            .destination_address_prefixes
            .get_output(context)
            .get_inner();
        let destination_application_security_group_ids_binding = args
            .destination_application_security_group_ids
            .get_output(context)
            .get_inner();
        let destination_port_range_binding = args
            .destination_port_range
            .get_output(context)
            .get_inner();
        let destination_port_ranges_binding = args
            .destination_port_ranges
            .get_output(context)
            .get_inner();
        let direction_binding = args.direction.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_security_group_name_binding = args
            .network_security_group_name
            .get_output(context)
            .get_inner();
        let priority_binding = args.priority.get_output(context).get_inner();
        let protocol_binding = args.protocol.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let source_address_prefix_binding = args
            .source_address_prefix
            .get_output(context)
            .get_inner();
        let source_address_prefixes_binding = args
            .source_address_prefixes
            .get_output(context)
            .get_inner();
        let source_application_security_group_ids_binding = args
            .source_application_security_group_ids
            .get_output(context)
            .get_inner();
        let source_port_range_binding = args
            .source_port_range
            .get_output(context)
            .get_inner();
        let source_port_ranges_binding = args
            .source_port_ranges
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/networkSecurityRule:NetworkSecurityRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "access".into(),
                    value: &access_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "destinationAddressPrefix".into(),
                    value: &destination_address_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "destinationAddressPrefixes".into(),
                    value: &destination_address_prefixes_binding,
                },
                register_interface::ObjectField {
                    name: "destinationApplicationSecurityGroupIds".into(),
                    value: &destination_application_security_group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "destinationPortRange".into(),
                    value: &destination_port_range_binding,
                },
                register_interface::ObjectField {
                    name: "destinationPortRanges".into(),
                    value: &destination_port_ranges_binding,
                },
                register_interface::ObjectField {
                    name: "direction".into(),
                    value: &direction_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkSecurityGroupName".into(),
                    value: &network_security_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sourceAddressPrefix".into(),
                    value: &source_address_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "sourceAddressPrefixes".into(),
                    value: &source_address_prefixes_binding,
                },
                register_interface::ObjectField {
                    name: "sourceApplicationSecurityGroupIds".into(),
                    value: &source_application_security_group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "sourcePortRange".into(),
                    value: &source_port_range_binding,
                },
                register_interface::ObjectField {
                    name: "sourcePortRanges".into(),
                    value: &source_port_ranges_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "access".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "destinationAddressPrefix".into(),
                },
                register_interface::ResultField {
                    name: "destinationAddressPrefixes".into(),
                },
                register_interface::ResultField {
                    name: "destinationApplicationSecurityGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "destinationPortRange".into(),
                },
                register_interface::ResultField {
                    name: "destinationPortRanges".into(),
                },
                register_interface::ResultField {
                    name: "direction".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkSecurityGroupName".into(),
                },
                register_interface::ResultField {
                    name: "priority".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sourceAddressPrefix".into(),
                },
                register_interface::ResultField {
                    name: "sourceAddressPrefixes".into(),
                },
                register_interface::ResultField {
                    name: "sourceApplicationSecurityGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "sourcePortRange".into(),
                },
                register_interface::ResultField {
                    name: "sourcePortRanges".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkSecurityRuleResult {
            access: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("access").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            destination_address_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationAddressPrefix").unwrap(),
            ),
            destination_address_prefixes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationAddressPrefixes").unwrap(),
            ),
            destination_application_security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationApplicationSecurityGroupIds").unwrap(),
            ),
            destination_port_range: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationPortRange").unwrap(),
            ),
            destination_port_ranges: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationPortRanges").unwrap(),
            ),
            direction: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("direction").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_security_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkSecurityGroupName").unwrap(),
            ),
            priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priority").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            source_address_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceAddressPrefix").unwrap(),
            ),
            source_address_prefixes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceAddressPrefixes").unwrap(),
            ),
            source_application_security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceApplicationSecurityGroupIds").unwrap(),
            ),
            source_port_range: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourcePortRange").unwrap(),
            ),
            source_port_ranges: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourcePortRanges").unwrap(),
            ),
        }
    }
}
