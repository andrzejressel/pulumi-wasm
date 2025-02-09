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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_manager_deployment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkManagerDeploymentArgs {
        /// A list of Network Manager Configuration IDs which should be aligned with `scope_access`.
        #[builder(into)]
        pub configuration_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Specifies the location which the configurations will be deployed to. Changing this forces a new Network Manager Deployment to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of the Network Manager. Changing this forces a new Network Manager Deployment to be created.
        #[builder(into)]
        pub network_manager_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the configuration deployment type. Possible values are `Connectivity` and `SecurityAdmin`. Changing this forces a new Network Manager Deployment to be created.
        #[builder(into)]
        pub scope_access: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of key values pairs that can be used to keep the deployment up with the Network Manager configurations and rules.
        #[builder(into, default)]
        pub triggers: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkManagerDeploymentResult {
        /// A list of Network Manager Configuration IDs which should be aligned with `scope_access`.
        pub configuration_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Specifies the location which the configurations will be deployed to. Changing this forces a new Network Manager Deployment to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the Network Manager. Changing this forces a new Network Manager Deployment to be created.
        pub network_manager_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the configuration deployment type. Possible values are `Connectivity` and `SecurityAdmin`. Changing this forces a new Network Manager Deployment to be created.
        pub scope_access: pulumi_gestalt_rust::Output<String>,
        /// A mapping of key values pairs that can be used to keep the deployment up with the Network Manager configurations and rules.
        pub triggers: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkManagerDeploymentArgs,
    ) -> NetworkManagerDeploymentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let configuration_ids_binding = args.configuration_ids.get_output(context);
        let location_binding = args.location.get_output(context);
        let network_manager_id_binding = args.network_manager_id.get_output(context);
        let scope_access_binding = args.scope_access.get_output(context);
        let triggers_binding = args.triggers.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/networkManagerDeployment:NetworkManagerDeployment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configurationIds".into(),
                    value: configuration_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkManagerId".into(),
                    value: network_manager_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scopeAccess".into(),
                    value: scope_access_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "triggers".into(),
                    value: triggers_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkManagerDeploymentResult {
            configuration_ids: o.get_field("configurationIds"),
            location: o.get_field("location"),
            network_manager_id: o.get_field("networkManagerId"),
            scope_access: o.get_field("scopeAccess"),
            triggers: o.get_field("triggers"),
        }
    }
}
