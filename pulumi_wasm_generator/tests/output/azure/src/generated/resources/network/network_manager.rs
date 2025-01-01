/// Manages a Network Managers.
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
///       tags:
///         foo: bar
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
/// ```
///
/// ## Import
///
/// Network Managers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/networkManager:NetworkManager example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Network/networkManagers/networkManager1
/// ```
///
pub mod network_manager {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkManagerArgs {
        /// A description of the network manager.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Azure Region where the Network Managers should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name which should be used for this Network Managers. Changing this forces a new Network Managers to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Resource Group where the Network Managers should exist. Changing this forces a new Network Managers to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `scope` block as defined below.
        #[builder(into)]
        pub scope: pulumi_wasm_rust::Output<
            super::super::types::network::NetworkManagerScope,
        >,
        /// A list of configuration deployment type. Possible values are `Connectivity`, `SecurityAdmin` and `Routing`, corresponds to if Connectivity Configuration, Security Admin Configuration or Routing Configuration is allowed for the Network Manager.
        #[builder(into)]
        pub scope_accesses: pulumi_wasm_rust::Output<Vec<String>>,
        /// A mapping of tags which should be assigned to the Network Managers.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkManagerResult {
        /// One or more `cross_tenant_scopes` blocks as defined below.
        pub cross_tenant_scopes: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::NetworkManagerCrossTenantScope>,
        >,
        /// A description of the network manager.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Azure Region where the Network Managers should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Network Managers. Changing this forces a new Network Managers to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Resource Group where the Network Managers should exist. Changing this forces a new Network Managers to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `scope` block as defined below.
        pub scope: pulumi_wasm_rust::Output<
            super::super::types::network::NetworkManagerScope,
        >,
        /// A list of configuration deployment type. Possible values are `Connectivity`, `SecurityAdmin` and `Routing`, corresponds to if Connectivity Configuration, Security Admin Configuration or Routing Configuration is allowed for the Network Manager.
        pub scope_accesses: pulumi_wasm_rust::Output<Vec<String>>,
        /// A mapping of tags which should be assigned to the Network Managers.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: NetworkManagerArgs) -> NetworkManagerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let scope_binding = args.scope.get_inner();
        let scope_accesses_binding = args.scope_accesses.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/networkManager:NetworkManager".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
                register_interface::ObjectField {
                    name: "scopeAccesses".into(),
                    value: &scope_accesses_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "crossTenantScopes".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "scope".into(),
                },
                register_interface::ResultField {
                    name: "scopeAccesses".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkManagerResult {
            cross_tenant_scopes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("crossTenantScopes").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scope").unwrap(),
            ),
            scope_accesses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scopeAccesses").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
