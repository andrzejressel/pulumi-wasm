/// Manages a Network Manager Connectivity Configuration.
///
/// > **Note:** The `azure.network.NetworkManagerConnectivityConfiguration` deployment may modify or delete existing Network Peering resource.
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
///   example2:
///     type: azure:network:NetworkManagerNetworkGroup
///     properties:
///       name: example-group2
///       networkManagerId: ${exampleNetworkManager.id}
///   exampleNetworkManagerConnectivityConfiguration:
///     type: azure:network:NetworkManagerConnectivityConfiguration
///     name: example
///     properties:
///       name: example-connectivity-conf
///       networkManagerId: ${exampleNetworkManager.id}
///       connectivityTopology: HubAndSpoke
///       appliesToGroups:
///         - groupConnectivity: DirectlyConnected
///           networkGroupId: ${exampleNetworkManagerNetworkGroup.id}
///         - groupConnectivity: DirectlyConnected
///           networkGroupId: ${example2.id}
///       hub:
///         resourceId: ${exampleVirtualNetwork.id}
///         resourceType: Microsoft.Network/virtualNetworks
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
/// ```
///
/// ## Import
///
/// Network Manager Connectivity Configuration can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/networkManagerConnectivityConfiguration:NetworkManagerConnectivityConfiguration example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Network/networkManagers/networkManager1/connectivityConfigurations/configuration1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_manager_connectivity_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkManagerConnectivityConfigurationArgs {
        /// One or more `applies_to_group` blocks as defined below.
        #[builder(into)]
        pub applies_to_groups: pulumi_gestalt_rust::InputOrOutput<
            Vec<
                super::super::types::network::NetworkManagerConnectivityConfigurationAppliesToGroup,
            >,
        >,
        /// Specifies the connectivity topology type. Possible values are `HubAndSpoke` and `Mesh`.
        #[builder(into)]
        pub connectivity_topology: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Indicates whether to remove current existing Virtual Network Peering in the Connectivity Configuration affected scope. Possible values are `true` and `false`.
        #[builder(into, default)]
        pub delete_existing_peering_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A description of the Connectivity Configuration.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates whether to global mesh is supported. Possible values are `true` and `false`.
        #[builder(into, default)]
        pub global_mesh_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `hub` block as defined below.
        #[builder(into, default)]
        pub hub: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::network::NetworkManagerConnectivityConfigurationHub,
            >,
        >,
        /// Specifies the name which should be used for this Network Manager Connectivity Configuration. Changing this forces a new Network Manager Connectivity Configuration to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of the Network Manager. Changing this forces a new Network Manager Connectivity Configuration to be created.
        #[builder(into)]
        pub network_manager_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkManagerConnectivityConfigurationResult {
        /// One or more `applies_to_group` blocks as defined below.
        pub applies_to_groups: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::network::NetworkManagerConnectivityConfigurationAppliesToGroup,
            >,
        >,
        /// Specifies the connectivity topology type. Possible values are `HubAndSpoke` and `Mesh`.
        pub connectivity_topology: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether to remove current existing Virtual Network Peering in the Connectivity Configuration affected scope. Possible values are `true` and `false`.
        pub delete_existing_peering_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A description of the Connectivity Configuration.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Indicates whether to global mesh is supported. Possible values are `true` and `false`.
        pub global_mesh_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `hub` block as defined below.
        pub hub: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::network::NetworkManagerConnectivityConfigurationHub,
            >,
        >,
        /// Specifies the name which should be used for this Network Manager Connectivity Configuration. Changing this forces a new Network Manager Connectivity Configuration to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the Network Manager. Changing this forces a new Network Manager Connectivity Configuration to be created.
        pub network_manager_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkManagerConnectivityConfigurationArgs,
    ) -> NetworkManagerConnectivityConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let applies_to_groups_binding = args.applies_to_groups.get_output(context);
        let connectivity_topology_binding = args
            .connectivity_topology
            .get_output(context);
        let delete_existing_peering_enabled_binding = args
            .delete_existing_peering_enabled
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let global_mesh_enabled_binding = args.global_mesh_enabled.get_output(context);
        let hub_binding = args.hub.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_manager_id_binding = args.network_manager_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/networkManagerConnectivityConfiguration:NetworkManagerConnectivityConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appliesToGroups".into(),
                    value: applies_to_groups_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectivityTopology".into(),
                    value: connectivity_topology_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deleteExistingPeeringEnabled".into(),
                    value: delete_existing_peering_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globalMeshEnabled".into(),
                    value: global_mesh_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hub".into(),
                    value: hub_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkManagerId".into(),
                    value: network_manager_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkManagerConnectivityConfigurationResult {
            applies_to_groups: o.get_field("appliesToGroups"),
            connectivity_topology: o.get_field("connectivityTopology"),
            delete_existing_peering_enabled: o.get_field("deleteExistingPeeringEnabled"),
            description: o.get_field("description"),
            global_mesh_enabled: o.get_field("globalMeshEnabled"),
            hub: o.get_field("hub"),
            name: o.get_field("name"),
            network_manager_id: o.get_field("networkManagerId"),
        }
    }
}
