/// Manages a Network Manager Static Member.
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
///       name: example-group
///       networkManagerId: ${exampleNetworkManager.id}
///       description: example network group
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       resourceGroupName: ${example.name}
///       addressSpaces:
///         - 192.168.1.0/24
///       location: ${example.location}
///   exampleNetworkManagerStaticMember:
///     type: azure:network:NetworkManagerStaticMember
///     name: example
///     properties:
///       name: example-nmsm
///       networkGroupId: ${exampleNetworkManagerNetworkGroup.id}
///       targetVirtualNetworkId: ${exampleVirtualNetwork.id}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
/// ```
///
/// ## Import
///
/// Network Manager Static Member can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/networkManagerStaticMember:NetworkManagerStaticMember example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Network/networkManagers/networkManager1/networkGroups/networkGroup1/staticMembers/staticMember1
/// ```
///
pub mod network_manager_static_member {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkManagerStaticMemberArgs {
        /// Specifies the name which should be used for this Network Manager Static Member. Changing this forces a new Network Manager Static Member to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of the Network Manager Group. Changing this forces a new Network Manager Static Member to be created.
        #[builder(into)]
        pub network_group_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the Resource ID of the Virtual Network using as the Static Member. Changing this forces a new Network Manager Static Member to be created.
        #[builder(into)]
        pub target_virtual_network_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkManagerStaticMemberResult {
        /// Specifies the name which should be used for this Network Manager Static Member. Changing this forces a new Network Manager Static Member to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the Network Manager Group. Changing this forces a new Network Manager Static Member to be created.
        pub network_group_id: pulumi_wasm_rust::Output<String>,
        /// The region of the Network Manager Static Member.
        pub region: pulumi_wasm_rust::Output<String>,
        /// Specifies the Resource ID of the Virtual Network using as the Static Member. Changing this forces a new Network Manager Static Member to be created.
        pub target_virtual_network_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NetworkManagerStaticMemberArgs,
    ) -> NetworkManagerStaticMemberResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let network_group_id_binding = args.network_group_id.get_inner();
        let target_virtual_network_id_binding = args
            .target_virtual_network_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/networkManagerStaticMember:NetworkManagerStaticMember"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkGroupId".into(),
                    value: &network_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "targetVirtualNetworkId".into(),
                    value: &target_virtual_network_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkGroupId".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "targetVirtualNetworkId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkManagerStaticMemberResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkGroupId").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            target_virtual_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetVirtualNetworkId").unwrap(),
            ),
        }
    }
}
