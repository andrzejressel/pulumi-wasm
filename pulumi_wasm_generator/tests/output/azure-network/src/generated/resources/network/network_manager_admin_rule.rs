/// Manages a Network Manager Admin Rule.
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
///   exampleNetworkManager:
///     type: azure:network:NetworkManager
///     name: example
///     properties:
///       name: example-network-manager
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       scope:
///         subscriptionIds:
///           - ${current.id}
///       scopeAccesses:
///         - Connectivity
///         - SecurityAdmin
///       description: example network manager
///   exampleNetworkManagerNetworkGroup:
///     type: azure:network:NetworkManagerNetworkGroup
///     name: example
///     properties:
///       name: example-network-group
///       networkManagerId: ${exampleNetworkManager.id}
///   exampleNetworkManagerSecurityAdminConfiguration:
///     type: azure:network:NetworkManagerSecurityAdminConfiguration
///     name: example
///     properties:
///       name: example-admin-conf
///       networkManagerId: ${exampleNetworkManager.id}
///   exampleNetworkManagerAdminRuleCollection:
///     type: azure:network:NetworkManagerAdminRuleCollection
///     name: example
///     properties:
///       name: example-admin-rule-collection
///       securityAdminConfigurationId: ${exampleNetworkManagerSecurityAdminConfiguration.id}
///       networkGroupIds:
///         - ${exampleNetworkManagerNetworkGroup.id}
///   exampleNetworkManagerAdminRule:
///     type: azure:network:NetworkManagerAdminRule
///     name: example
///     properties:
///       name: example-admin-rule
///       adminRuleCollectionId: ${exampleNetworkManagerAdminRuleCollection.id}
///       action: Deny
///       direction: Outbound
///       priority: 1
///       protocol: Tcp
///       sourcePortRanges:
///         - '80'
///         - 1024-65535
///       destinationPortRanges:
///         - '80'
///       sources:
///         - addressPrefixType: ServiceTag
///           addressPrefix: Internet
///       destinations:
///         - addressPrefixType: IPPrefix
///           addressPrefix: 10.1.0.1
///         - addressPrefixType: IPPrefix
///           addressPrefix: 10.0.0.0/24
///       description: example admin rule
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
/// ```
///
/// ## Import
///
/// Network Manager Admin Rule can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/networkManagerAdminRule:NetworkManagerAdminRule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Network/networkManagers/networkManager1/securityAdminConfigurations/configuration1/ruleCollections/ruleCollection1/rules/rule1
/// ```
///
pub mod network_manager_admin_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkManagerAdminRuleArgs {
        /// Specifies the action allowed for this Network Manager Admin Rule. Possible values are `Allow`, `AlwaysAllow`, and `Deny`.
        #[builder(into)]
        pub action: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the Network Manager Admin Rule Collection. Changing this forces a new Network Manager Admin Rule to be created.
        #[builder(into)]
        pub admin_rule_collection_id: pulumi_wasm_rust::Output<String>,
        /// A description of the Network Manager Admin Rule.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of string specifies the destination port ranges. Specify one or more single port number or port ranges such as `1024-65535`. Use `*` to specify any port.
        #[builder(into, default)]
        pub destination_port_ranges: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// One or more `destination` blocks as defined below.
        #[builder(into, default)]
        pub destinations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::NetworkManagerAdminRuleDestination>>,
        >,
        /// Indicates if the traffic matched against the rule in inbound or outbound. Possible values are `Inbound` and `Outbound`.
        #[builder(into)]
        pub direction: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Network Manager Admin Rule. Changing this forces a new Network Manager Admin Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The priority of the rule. Possible values are integers between `1` and `4096`. The priority number must be unique for each rule in the collection. The lower the priority number, the higher the priority of the rule.
        #[builder(into)]
        pub priority: pulumi_wasm_rust::Output<i32>,
        /// Specifies which network protocol this Network Manager Admin Rule applies to. Possible values are `Ah`, `Any`, `Esp`, `Icmp`, `Tcp`, and `Udp`.
        #[builder(into)]
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// A list of string specifies the source port ranges. Specify one or more single port number or port ranges such as `1024-65535`. Use `*` to specify any port.
        #[builder(into, default)]
        pub source_port_ranges: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// One or more `source` blocks as defined below.
        #[builder(into, default)]
        pub sources: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::NetworkManagerAdminRuleSource>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkManagerAdminRuleResult {
        /// Specifies the action allowed for this Network Manager Admin Rule. Possible values are `Allow`, `AlwaysAllow`, and `Deny`.
        pub action: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the Network Manager Admin Rule Collection. Changing this forces a new Network Manager Admin Rule to be created.
        pub admin_rule_collection_id: pulumi_wasm_rust::Output<String>,
        /// A description of the Network Manager Admin Rule.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of string specifies the destination port ranges. Specify one or more single port number or port ranges such as `1024-65535`. Use `*` to specify any port.
        pub destination_port_ranges: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// One or more `destination` blocks as defined below.
        pub destinations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::NetworkManagerAdminRuleDestination>>,
        >,
        /// Indicates if the traffic matched against the rule in inbound or outbound. Possible values are `Inbound` and `Outbound`.
        pub direction: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Network Manager Admin Rule. Changing this forces a new Network Manager Admin Rule to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The priority of the rule. Possible values are integers between `1` and `4096`. The priority number must be unique for each rule in the collection. The lower the priority number, the higher the priority of the rule.
        pub priority: pulumi_wasm_rust::Output<i32>,
        /// Specifies which network protocol this Network Manager Admin Rule applies to. Possible values are `Ah`, `Any`, `Esp`, `Icmp`, `Tcp`, and `Udp`.
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// A list of string specifies the source port ranges. Specify one or more single port number or port ranges such as `1024-65535`. Use `*` to specify any port.
        pub source_port_ranges: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// One or more `source` blocks as defined below.
        pub sources: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::NetworkManagerAdminRuleSource>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NetworkManagerAdminRuleArgs,
    ) -> NetworkManagerAdminRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_inner();
        let admin_rule_collection_id_binding = args.admin_rule_collection_id.get_inner();
        let description_binding = args.description.get_inner();
        let destination_port_ranges_binding = args.destination_port_ranges.get_inner();
        let destinations_binding = args.destinations.get_inner();
        let direction_binding = args.direction.get_inner();
        let name_binding = args.name.get_inner();
        let priority_binding = args.priority.get_inner();
        let protocol_binding = args.protocol.get_inner();
        let source_port_ranges_binding = args.source_port_ranges.get_inner();
        let sources_binding = args.sources.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/networkManagerAdminRule:NetworkManagerAdminRule"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "action".into(),
                    value: &action_binding,
                },
                register_interface::ObjectField {
                    name: "adminRuleCollectionId".into(),
                    value: &admin_rule_collection_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "destinationPortRanges".into(),
                    value: &destination_port_ranges_binding,
                },
                register_interface::ObjectField {
                    name: "destinations".into(),
                    value: &destinations_binding,
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
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding,
                },
                register_interface::ObjectField {
                    name: "sourcePortRanges".into(),
                    value: &source_port_ranges_binding,
                },
                register_interface::ObjectField {
                    name: "sources".into(),
                    value: &sources_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "action".into(),
                },
                register_interface::ResultField {
                    name: "adminRuleCollectionId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "destinationPortRanges".into(),
                },
                register_interface::ResultField {
                    name: "destinations".into(),
                },
                register_interface::ResultField {
                    name: "direction".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "priority".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
                register_interface::ResultField {
                    name: "sourcePortRanges".into(),
                },
                register_interface::ResultField {
                    name: "sources".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkManagerAdminRuleResult {
            action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("action").unwrap(),
            ),
            admin_rule_collection_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("adminRuleCollectionId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            destination_port_ranges: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationPortRanges").unwrap(),
            ),
            destinations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinations").unwrap(),
            ),
            direction: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("direction").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priority").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
            source_port_ranges: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourcePortRanges").unwrap(),
            ),
            sources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sources").unwrap(),
            ),
        }
    }
}
