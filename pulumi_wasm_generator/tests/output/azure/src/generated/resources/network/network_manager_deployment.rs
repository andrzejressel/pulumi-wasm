/// Manages a Network Manager Deployment.
///
/// > **NOTE on Virtual Network Peering:** Using Network Manager Deployment to deploy Connectivity Configuration may modify or delete existing Virtual Network Peering. At this time you should not use Network Peering resource in conjunction with Network Manager Deployment. Doing so may cause a conflict of Peering configurations.
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
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-net
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       addressSpaces:
///         - 10.0.0.0/16
///       flowTimeoutInMinutes: 10
///   exampleNetworkManagerConnectivityConfiguration:
///     type: azure:network:NetworkManagerConnectivityConfiguration
///     name: example
///     properties:
///       name: example-connectivity-conf
///       networkManagerId: ${exampleNetworkManager.id}
///       connectivityTopology: HubAndSpoke
///       appliesToGroups:
///         - groupConnectivity: None
///           networkGroupId: ${exampleNetworkManagerNetworkGroup.id}
///       hub:
///         resourceId: ${exampleVirtualNetwork.id}
///         resourceType: Microsoft.Network/virtualNetworks
///   exampleNetworkManagerDeployment:
///     type: azure:network:NetworkManagerDeployment
///     name: example
///     properties:
///       networkManagerId: ${exampleNetworkManager.id}
///       location: eastus
///       scopeAccess: Connectivity
///       configurationIds:
///         - ${exampleNetworkManagerConnectivityConfiguration.id}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
/// ```
///
///
/// ### Triggers)
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
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-net
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       addressSpaces:
///         - 10.0.0.0/16
///       flowTimeoutInMinutes: 10
///   exampleNetworkManagerSecurityAdminConfiguration:
///     type: azure:network:NetworkManagerSecurityAdminConfiguration
///     name: example
///     properties:
///       name: example-nmsac
///       networkManagerId: ${exampleNetworkManager.id}
///   exampleNetworkManagerAdminRuleCollection:
///     type: azure:network:NetworkManagerAdminRuleCollection
///     name: example
///     properties:
///       name: example-nmarc
///       securityAdminConfigurationId: ${exampleNetworkManagerSecurityAdminConfiguration.id}
///       networkGroupIds:
///         - ${exampleNetworkManagerNetworkGroup.id}
///   exampleNetworkManagerAdminRule:
///     type: azure:network:NetworkManagerAdminRule
///     name: example
///     properties:
///       name: example-nmar
///       adminRuleCollectionId: ${exampleNetworkManagerAdminRuleCollection.id}
///       action: Deny
///       description: example
///       direction: Inbound
///       priority: 1
///       protocol: Tcp
///       sourcePortRanges:
///         - '80'
///       destinationPortRanges:
///         - '80'
///       sources:
///         - addressPrefixType: ServiceTag
///           addressPrefix: Internet
///       destinations:
///         - addressPrefixType: IPPrefix
///           addressPrefix: '*'
///   exampleNetworkManagerDeployment:
///     type: azure:network:NetworkManagerDeployment
///     name: example
///     properties:
///       networkManagerId: ${exampleNetworkManager.id}
///       location: eastus
///       scopeAccess: SecurityAdmin
///       configurationIds:
///         - ${exampleNetworkManagerSecurityAdminConfiguration.id}
///       triggers:
///         source_port_ranges:
///           fn::invoke:
///             function: std:join
///             arguments:
///               separator: ','
///               input: ${exampleNetworkManagerAdminRule.sourcePortRanges}
///             return: result
///     options:
///       dependsOn:
///         - ${exampleNetworkManagerAdminRule}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
/// ```
///
/// ## Import
///
/// Network Manager Deployment can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/networkManagerDeployment:NetworkManagerDeployment example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Network/networkManagers/networkManager1/commit|eastus|Connectivity
/// ```
///
pub mod network_manager_deployment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkManagerDeploymentArgs {
        /// A list of Network Manager Configuration IDs which should be aligned with `scope_access`.
        #[builder(into)]
        pub configuration_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the location which the configurations will be deployed to. Changing this forces a new Network Manager Deployment to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of the Network Manager. Changing this forces a new Network Manager Deployment to be created.
        #[builder(into)]
        pub network_manager_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the configuration deployment type. Possible values are `Connectivity` and `SecurityAdmin`. Changing this forces a new Network Manager Deployment to be created.
        #[builder(into)]
        pub scope_access: pulumi_wasm_rust::Output<String>,
        /// A mapping of key values pairs that can be used to keep the deployment up with the Network Manager configurations and rules.
        #[builder(into, default)]
        pub triggers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkManagerDeploymentResult {
        /// A list of Network Manager Configuration IDs which should be aligned with `scope_access`.
        pub configuration_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the location which the configurations will be deployed to. Changing this forces a new Network Manager Deployment to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the Network Manager. Changing this forces a new Network Manager Deployment to be created.
        pub network_manager_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the configuration deployment type. Possible values are `Connectivity` and `SecurityAdmin`. Changing this forces a new Network Manager Deployment to be created.
        pub scope_access: pulumi_wasm_rust::Output<String>,
        /// A mapping of key values pairs that can be used to keep the deployment up with the Network Manager configurations and rules.
        pub triggers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NetworkManagerDeploymentArgs,
    ) -> NetworkManagerDeploymentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let configuration_ids_binding = args.configuration_ids.get_inner();
        let location_binding = args.location.get_inner();
        let network_manager_id_binding = args.network_manager_id.get_inner();
        let scope_access_binding = args.scope_access.get_inner();
        let triggers_binding = args.triggers.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/networkManagerDeployment:NetworkManagerDeployment"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configurationIds".into(),
                    value: &configuration_ids_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "networkManagerId".into(),
                    value: &network_manager_id_binding,
                },
                register_interface::ObjectField {
                    name: "scopeAccess".into(),
                    value: &scope_access_binding,
                },
                register_interface::ObjectField {
                    name: "triggers".into(),
                    value: &triggers_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "configurationIds".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "networkManagerId".into(),
                },
                register_interface::ResultField {
                    name: "scopeAccess".into(),
                },
                register_interface::ResultField {
                    name: "triggers".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkManagerDeploymentResult {
            configuration_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationIds").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            network_manager_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkManagerId").unwrap(),
            ),
            scope_access: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scopeAccess").unwrap(),
            ),
            triggers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("triggers").unwrap(),
            ),
        }
    }
}