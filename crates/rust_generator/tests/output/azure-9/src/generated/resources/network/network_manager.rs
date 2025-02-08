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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_manager {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkManagerArgs {
        /// A description of the network manager.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Azure Region where the Network Managers should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name which should be used for this Network Managers. Changing this forces a new Network Managers to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Resource Group where the Network Managers should exist. Changing this forces a new Network Managers to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `scope` block as defined below.
        #[builder(into)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::network::NetworkManagerScope,
        >,
        /// A list of configuration deployment type. Possible values are `Connectivity`, `SecurityAdmin` and `Routing`, corresponds to if Connectivity Configuration, Security Admin Configuration or Routing Configuration is allowed for the Network Manager.
        #[builder(into)]
        pub scope_accesses: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// A mapping of tags which should be assigned to the Network Managers.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkManagerResult {
        /// One or more `cross_tenant_scopes` blocks as defined below.
        pub cross_tenant_scopes: pulumi_gestalt_rust::Output<
            Vec<super::super::types::network::NetworkManagerCrossTenantScope>,
        >,
        /// A description of the network manager.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the Azure Region where the Network Managers should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this Network Managers. Changing this forces a new Network Managers to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Resource Group where the Network Managers should exist. Changing this forces a new Network Managers to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `scope` block as defined below.
        pub scope: pulumi_gestalt_rust::Output<
            super::super::types::network::NetworkManagerScope,
        >,
        /// A list of configuration deployment type. Possible values are `Connectivity`, `SecurityAdmin` and `Routing`, corresponds to if Connectivity Configuration, Security Admin Configuration or Routing Configuration is allowed for the Network Manager.
        pub scope_accesses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A mapping of tags which should be assigned to the Network Managers.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: NetworkManagerArgs,
    ) -> NetworkManagerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let scope_binding = args.scope.get_output(context).get_inner();
        let scope_accesses_binding = args.scope_accesses.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/networkManager:NetworkManager".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        NetworkManagerResult {
            cross_tenant_scopes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("crossTenantScopes"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            scope: pulumi_gestalt_rust::__private::into_domain(o.extract_field("scope")),
            scope_accesses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scopeAccesses"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
