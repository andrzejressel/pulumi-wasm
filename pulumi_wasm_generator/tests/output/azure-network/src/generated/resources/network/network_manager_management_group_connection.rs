/// Manages a Network Manager Management Group Connection which may cross tenants.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:management:Group
///   exampleGroupSubscriptionAssociation:
///     type: azure:management:GroupSubscriptionAssociation
///     name: example
///     properties:
///       managementGroupId: ${example.id}
///       subscriptionId: ${alt.id}
///   networkContributor:
///     type: azure:authorization:Assignment
///     name: network_contributor
///     properties:
///       scope: ${example.id}
///       roleDefinitionName: Network Contributor
///       principalId: ${currentGetClientConfig.objectId}
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleNetworkManager:
///     type: azure:network:NetworkManager
///     name: example
///     properties:
///       name: example-networkmanager
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       scope:
///         subscriptionIds:
///           - ${current.id}
///       scopeAccesses:
///         - SecurityAdmin
///   exampleNetworkManagerManagementGroupConnection:
///     type: azure:network:NetworkManagerManagementGroupConnection
///     name: example
///     properties:
///       name: example-nmmgc
///       managementGroupId: ${example.id}
///       networkManagerId: ${exampleNetworkManager.id}
///       description: example
///     options:
///       dependsOn:
///         - ${networkContributor}
/// variables:
///   alt:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments:
///         subscriptionId: 00000000-0000-0000-0000-000000000000
///   current:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
///   currentGetClientConfig:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Network Manager Management Group Connection can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/networkManagerManagementGroupConnection:NetworkManagerManagementGroupConnection example /providers/Microsoft.Management/managementGroups/00000000-0000-0000-0000-000000000000/providers/Microsoft.Network/networkManagerConnections/networkManagerConnection1
/// ```
///
pub mod network_manager_management_group_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkManagerManagementGroupConnectionArgs {
        /// A description of the Network Manager Management Group Connection.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of the target Management Group. Changing this forces a new resource to be created.
        #[builder(into)]
        pub management_group_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Network Manager Management Group Connection. Changing this forces a new Network Manager Management Group Connection to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of the Network Manager which the Management Group is connected to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub network_manager_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkManagerManagementGroupConnectionResult {
        /// The Connection state of the Network Manager Management Group Connection.
        pub connection_state: pulumi_wasm_rust::Output<String>,
        /// A description of the Network Manager Management Group Connection.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of the target Management Group. Changing this forces a new resource to be created.
        pub management_group_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Network Manager Management Group Connection. Changing this forces a new Network Manager Management Group Connection to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the Network Manager which the Management Group is connected to. Changing this forces a new resource to be created.
        pub network_manager_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NetworkManagerManagementGroupConnectionArgs,
    ) -> NetworkManagerManagementGroupConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let management_group_id_binding = args.management_group_id.get_inner();
        let name_binding = args.name.get_inner();
        let network_manager_id_binding = args.network_manager_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/networkManagerManagementGroupConnection:NetworkManagerManagementGroupConnection"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "managementGroupId".into(),
                    value: &management_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkManagerId".into(),
                    value: &network_manager_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "connectionState".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "managementGroupId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkManagerId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkManagerManagementGroupConnectionResult {
            connection_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionState").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            management_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managementGroupId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_manager_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkManagerId").unwrap(),
            ),
        }
    }
}
