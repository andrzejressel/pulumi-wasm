/// Manages an Azure Monitor Workspace.
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
///   exampleWorkspace:
///     type: azure:monitoring:Workspace
///     name: example
///     properties:
///       name: example-mamw
///       resourceGroupName: ${example.name}
///       location: West Europe
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// Azure Monitor Workspace can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:monitoring/workspace:Workspace example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Monitor/accounts/azureMonitorWorkspace1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workspace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceArgs {
        /// Specifies the Azure Region where the Azure Monitor Workspace should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name which should be used for this Azure Monitor Workspace. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Is public network access enabled? Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the name of the Resource Group where the Azure Monitor Workspace should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Azure Monitor Workspace.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct WorkspaceResult {
        /// The ID of the managed default Data Collection Endpoint created with the Azure Monitor Workspace.
        pub default_data_collection_endpoint_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the managed default Data Collection Rule created with the Azure Monitor Workspace.
        pub default_data_collection_rule_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Azure Region where the Azure Monitor Workspace should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this Azure Monitor Workspace. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Is public network access enabled? Defaults to `true`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The query endpoint for the Azure Monitor Workspace.
        pub query_endpoint: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Resource Group where the Azure Monitor Workspace should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Azure Monitor Workspace.
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
        args: WorkspaceArgs,
    ) -> WorkspaceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let public_network_access_enabled_binding_1 = args
            .public_network_access_enabled
            .get_output(context);
        let public_network_access_enabled_binding = public_network_access_enabled_binding_1
            .get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:monitoring/workspace:Workspace".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        WorkspaceResult {
            default_data_collection_endpoint_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultDataCollectionEndpointId"),
            ),
            default_data_collection_rule_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultDataCollectionRuleId"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            public_network_access_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicNetworkAccessEnabled"),
            ),
            query_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("queryEndpoint"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
