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
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkManagerAdminRuleArgs {
        /// Specifies the action allowed for this Network Manager Admin Rule. Possible values are `Allow`, `AlwaysAllow`, and `Deny`.
        #[builder(into)]
        pub action: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the ID of the Network Manager Admin Rule Collection. Changing this forces a new Network Manager Admin Rule to be created.
        #[builder(into)]
        pub admin_rule_collection_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A description of the Network Manager Admin Rule.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of string specifies the destination port ranges. Specify one or more single port number or port ranges such as `1024-65535`. Use `*` to specify any port.
        #[builder(into, default)]
        pub destination_port_ranges: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// One or more `destination` blocks as defined below.
        #[builder(into, default)]
        pub destinations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::network::NetworkManagerAdminRuleDestination>>,
        >,
        /// Indicates if the traffic matched against the rule in inbound or outbound. Possible values are `Inbound` and `Outbound`.
        #[builder(into)]
        pub direction: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name which should be used for this Network Manager Admin Rule. Changing this forces a new Network Manager Admin Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The priority of the rule. Possible values are integers between `1` and `4096`. The priority number must be unique for each rule in the collection. The lower the priority number, the higher the priority of the rule.
        #[builder(into)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Specifies which network protocol this Network Manager Admin Rule applies to. Possible values are `Ah`, `Any`, `Esp`, `Icmp`, `Tcp`, and `Udp`.
        #[builder(into)]
        pub protocol: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of string specifies the source port ranges. Specify one or more single port number or port ranges such as `1024-65535`. Use `*` to specify any port.
        #[builder(into, default)]
        pub source_port_ranges: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// One or more `source` blocks as defined below.
        #[builder(into, default)]
        pub sources: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::network::NetworkManagerAdminRuleSource>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkManagerAdminRuleResult {
        /// Specifies the action allowed for this Network Manager Admin Rule. Possible values are `Allow`, `AlwaysAllow`, and `Deny`.
        pub action: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the Network Manager Admin Rule Collection. Changing this forces a new Network Manager Admin Rule to be created.
        pub admin_rule_collection_id: pulumi_gestalt_rust::Output<String>,
        /// A description of the Network Manager Admin Rule.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of string specifies the destination port ranges. Specify one or more single port number or port ranges such as `1024-65535`. Use `*` to specify any port.
        pub destination_port_ranges: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// One or more `destination` blocks as defined below.
        pub destinations: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::network::NetworkManagerAdminRuleDestination>>,
        >,
        /// Indicates if the traffic matched against the rule in inbound or outbound. Possible values are `Inbound` and `Outbound`.
        pub direction: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this Network Manager Admin Rule. Changing this forces a new Network Manager Admin Rule to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The priority of the rule. Possible values are integers between `1` and `4096`. The priority number must be unique for each rule in the collection. The lower the priority number, the higher the priority of the rule.
        pub priority: pulumi_gestalt_rust::Output<i32>,
        /// Specifies which network protocol this Network Manager Admin Rule applies to. Possible values are `Ah`, `Any`, `Esp`, `Icmp`, `Tcp`, and `Udp`.
        pub protocol: pulumi_gestalt_rust::Output<String>,
        /// A list of string specifies the source port ranges. Specify one or more single port number or port ranges such as `1024-65535`. Use `*` to specify any port.
        pub source_port_ranges: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// One or more `source` blocks as defined below.
        pub sources: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::network::NetworkManagerAdminRuleSource>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: NetworkManagerAdminRuleArgs,
    ) -> NetworkManagerAdminRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_output(context).get_inner();
        let admin_rule_collection_id_binding = args
            .admin_rule_collection_id
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let destination_port_ranges_binding = args
            .destination_port_ranges
            .get_output(context)
            .get_inner();
        let destinations_binding = args.destinations.get_output(context).get_inner();
        let direction_binding = args.direction.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let priority_binding = args.priority.get_output(context).get_inner();
        let protocol_binding = args.protocol.get_output(context).get_inner();
        let source_port_ranges_binding = args
            .source_port_ranges
            .get_output(context)
            .get_inner();
        let sources_binding = args.sources.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/networkManagerAdminRule:NetworkManagerAdminRule"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        NetworkManagerAdminRuleResult {
            action: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("action"),
            ),
            admin_rule_collection_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("adminRuleCollectionId"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            destination_port_ranges: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destinationPortRanges"),
            ),
            destinations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destinations"),
            ),
            direction: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("direction"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            priority: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("priority"),
            ),
            protocol: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("protocol"),
            ),
            source_port_ranges: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourcePortRanges"),
            ),
            sources: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sources"),
            ),
        }
    }
}
