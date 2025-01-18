/// Manages a Network Manager Scope Connection which may cross tenants.
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
///       name: example-networkmanager
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       scope:
///         subscriptionIds:
///           - ${currentGetSubscription.id}
///       scopeAccesses:
///         - SecurityAdmin
///   exampleNetworkManagerScopeConnection:
///     type: azure:network:NetworkManagerScopeConnection
///     name: example
///     properties:
///       name: example-nsc
///       networkManagerId: ${exampleNetworkManager.id}
///       tenantId: ${current.tenantId}
///       targetScopeId: ${alt.id}
///       description: example
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
///   currentGetSubscription:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
///   alt:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments:
///         subscriptionId: 00000000-0000-0000-0000-000000000000
/// ```
///
/// ## Import
///
/// Network Manager Scope Connection can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/networkManagerScopeConnection:NetworkManagerScopeConnection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Network/networkManagers/networkManager1/scopeConnections/scopeConnection1
/// ```
///
pub mod network_manager_scope_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkManagerScopeConnectionArgs {
        /// A description of the Network Manager Scope Connection.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name which should be used for this Network Manager Scope Connection. Changing this forces a new Network Manager Scope Connection to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of the Network Manager Scope Connection. Changing this forces a new Network Manager Scope Connection to be created.
        #[builder(into)]
        pub network_manager_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the Resource ID of the target scope which the Network Manager is connected to. It should be either Subscription ID or Management Group ID.
        #[builder(into)]
        pub target_scope_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the Tenant ID of the Resource which the Network Manager is connected to.
        #[builder(into)]
        pub tenant_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkManagerScopeConnectionResult {
        /// The Connection state of the Network Manager Scope Connection.
        pub connection_state: pulumi_wasm_rust::Output<String>,
        /// A description of the Network Manager Scope Connection.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name which should be used for this Network Manager Scope Connection. Changing this forces a new Network Manager Scope Connection to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the Network Manager Scope Connection. Changing this forces a new Network Manager Scope Connection to be created.
        pub network_manager_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the Resource ID of the target scope which the Network Manager is connected to. It should be either Subscription ID or Management Group ID.
        pub target_scope_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the Tenant ID of the Resource which the Network Manager is connected to.
        pub tenant_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NetworkManagerScopeConnectionArgs,
    ) -> NetworkManagerScopeConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let network_manager_id_binding = args.network_manager_id.get_inner();
        let target_scope_id_binding = args.target_scope_id.get_inner();
        let tenant_id_binding = args.tenant_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/networkManagerScopeConnection:NetworkManagerScopeConnection"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkManagerId".into(),
                    value: &network_manager_id_binding,
                },
                register_interface::ObjectField {
                    name: "targetScopeId".into(),
                    value: &target_scope_id_binding,
                },
                register_interface::ObjectField {
                    name: "tenantId".into(),
                    value: &tenant_id_binding,
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
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkManagerId".into(),
                },
                register_interface::ResultField {
                    name: "targetScopeId".into(),
                },
                register_interface::ResultField {
                    name: "tenantId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkManagerScopeConnectionResult {
            connection_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionState").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_manager_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkManagerId").unwrap(),
            ),
            target_scope_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetScopeId").unwrap(),
            ),
            tenant_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenantId").unwrap(),
            ),
        }
    }
}
