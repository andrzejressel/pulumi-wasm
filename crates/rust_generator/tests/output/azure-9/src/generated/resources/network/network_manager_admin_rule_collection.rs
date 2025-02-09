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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_manager_admin_rule_collection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkManagerAdminRuleCollectionArgs {
        /// A description of the Network Manager Admin Rule Collection.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name which should be used for this Network Manager Admin Rule Collection. Changing this forces a new Network Manager Admin Rule Collection to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of Network Group ID which this Network Manager Admin Rule Collection applies to.
        #[builder(into)]
        pub network_group_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Specifies the ID of the Network Manager Security Admin Configuration. Changing this forces a new Network Manager Admin Rule Collection to be created.
        #[builder(into)]
        pub security_admin_configuration_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkManagerAdminRuleCollectionResult {
        /// A description of the Network Manager Admin Rule Collection.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name which should be used for this Network Manager Admin Rule Collection. Changing this forces a new Network Manager Admin Rule Collection to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of Network Group ID which this Network Manager Admin Rule Collection applies to.
        pub network_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Specifies the ID of the Network Manager Security Admin Configuration. Changing this forces a new Network Manager Admin Rule Collection to be created.
        pub security_admin_configuration_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkManagerAdminRuleCollectionArgs,
    ) -> NetworkManagerAdminRuleCollectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_group_ids_binding = args.network_group_ids.get_output(context);
        let security_admin_configuration_id_binding = args
            .security_admin_configuration_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/networkManagerAdminRuleCollection:NetworkManagerAdminRuleCollection"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkGroupIds".into(),
                    value: network_group_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityAdminConfigurationId".into(),
                    value: security_admin_configuration_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkManagerAdminRuleCollectionResult {
            description: o.get_field("description"),
            name: o.get_field("name"),
            network_group_ids: o.get_field("networkGroupIds"),
            security_admin_configuration_id: o.get_field("securityAdminConfigurationId"),
        }
    }
}
