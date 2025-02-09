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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_manager_management_group_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkManagerManagementGroupConnectionArgs {
        /// A description of the Network Manager Management Group Connection.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of the target Management Group. Changing this forces a new resource to be created.
        #[builder(into)]
        pub management_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name which should be used for this Network Manager Management Group Connection. Changing this forces a new Network Manager Management Group Connection to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of the Network Manager which the Management Group is connected to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub network_manager_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkManagerManagementGroupConnectionResult {
        /// The Connection state of the Network Manager Management Group Connection.
        pub connection_state: pulumi_gestalt_rust::Output<String>,
        /// A description of the Network Manager Management Group Connection.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the ID of the target Management Group. Changing this forces a new resource to be created.
        pub management_group_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this Network Manager Management Group Connection. Changing this forces a new Network Manager Management Group Connection to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the Network Manager which the Management Group is connected to. Changing this forces a new resource to be created.
        pub network_manager_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkManagerManagementGroupConnectionArgs,
    ) -> NetworkManagerManagementGroupConnectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let management_group_id_binding = args.management_group_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_manager_id_binding = args.network_manager_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/networkManagerManagementGroupConnection:NetworkManagerManagementGroupConnection"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managementGroupId".into(),
                    value: management_group_id_binding.get_id(),
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
        NetworkManagerManagementGroupConnectionResult {
            connection_state: o.get_field("connectionState"),
            description: o.get_field("description"),
            management_group_id: o.get_field("managementGroupId"),
            name: o.get_field("name"),
            network_manager_id: o.get_field("networkManagerId"),
        }
    }
}
