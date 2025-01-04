/// Manages a Network Manager Admin Rule Collection.
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
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
/// ```
///
/// ## Import
///
/// Network Manager Admin Rule Collection can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/networkManagerAdminRuleCollection:NetworkManagerAdminRuleCollection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Network/networkManagers/networkManager1/securityAdminConfigurations/configuration1/ruleCollections/ruleCollection1
/// ```
///
pub mod network_manager_admin_rule_collection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkManagerAdminRuleCollectionArgs {
        /// A description of the Network Manager Admin Rule Collection.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name which should be used for this Network Manager Admin Rule Collection. Changing this forces a new Network Manager Admin Rule Collection to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of Network Group ID which this Network Manager Admin Rule Collection applies to.
        #[builder(into)]
        pub network_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the ID of the Network Manager Security Admin Configuration. Changing this forces a new Network Manager Admin Rule Collection to be created.
        #[builder(into)]
        pub security_admin_configuration_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkManagerAdminRuleCollectionResult {
        /// A description of the Network Manager Admin Rule Collection.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name which should be used for this Network Manager Admin Rule Collection. Changing this forces a new Network Manager Admin Rule Collection to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of Network Group ID which this Network Manager Admin Rule Collection applies to.
        pub network_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the ID of the Network Manager Security Admin Configuration. Changing this forces a new Network Manager Admin Rule Collection to be created.
        pub security_admin_configuration_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NetworkManagerAdminRuleCollectionArgs,
    ) -> NetworkManagerAdminRuleCollectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let network_group_ids_binding = args.network_group_ids.get_inner();
        let security_admin_configuration_id_binding = args
            .security_admin_configuration_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/networkManagerAdminRuleCollection:NetworkManagerAdminRuleCollection"
                .into(),
            name: name.to_string(),
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
                    name: "networkGroupIds".into(),
                    value: &network_group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "securityAdminConfigurationId".into(),
                    value: &security_admin_configuration_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "securityAdminConfigurationId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkManagerAdminRuleCollectionResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkGroupIds").unwrap(),
            ),
            security_admin_configuration_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityAdminConfigurationId").unwrap(),
            ),
        }
    }
}
