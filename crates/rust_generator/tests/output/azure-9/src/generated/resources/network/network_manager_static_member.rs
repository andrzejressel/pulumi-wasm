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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_manager_static_member {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkManagerStaticMemberArgs {
        /// Specifies the name which should be used for this Network Manager Static Member. Changing this forces a new Network Manager Static Member to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of the Network Manager Group. Changing this forces a new Network Manager Static Member to be created.
        #[builder(into)]
        pub network_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Resource ID of the Virtual Network using as the Static Member. Changing this forces a new Network Manager Static Member to be created.
        #[builder(into)]
        pub target_virtual_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkManagerStaticMemberResult {
        /// Specifies the name which should be used for this Network Manager Static Member. Changing this forces a new Network Manager Static Member to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the Network Manager Group. Changing this forces a new Network Manager Static Member to be created.
        pub network_group_id: pulumi_gestalt_rust::Output<String>,
        /// The region of the Network Manager Static Member.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Resource ID of the Virtual Network using as the Static Member. Changing this forces a new Network Manager Static Member to be created.
        pub target_virtual_network_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkManagerStaticMemberArgs,
    ) -> NetworkManagerStaticMemberResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let network_group_id_binding = args.network_group_id.get_output(context);
        let target_virtual_network_id_binding = args
            .target_virtual_network_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/networkManagerStaticMember:NetworkManagerStaticMember"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkGroupId".into(),
                    value: network_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetVirtualNetworkId".into(),
                    value: target_virtual_network_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkManagerStaticMemberResult {
            name: o.get_field("name"),
            network_group_id: o.get_field("networkGroupId"),
            region: o.get_field("region"),
            target_virtual_network_id: o.get_field("targetVirtualNetworkId"),
        }
    }
}
