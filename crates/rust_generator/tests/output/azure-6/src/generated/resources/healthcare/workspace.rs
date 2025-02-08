/// Manages a Healthcare workspace
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = workspace::create(
///         "test",
///         WorkspaceArgs::builder()
///             .location("east us")
///             .name("tfexworkspace")
///             .resource_group_name("tfex-resource_group")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Healthcare Workspaces can be imported using the resource`id`, e.g.
///
/// ```sh
/// $ pulumi import azure:healthcare/workspace:Workspace example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.HealthcareApis/workspaces/workspace1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workspace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceArgs {
        /// Specifies the Azure Region where the Healthcare Workspace should be created. Changing this forces a new Healthcare Workspace to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Healthcare Workspace. Changing this forces a new Healthcare Workspace to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Resource Group where the Healthcare Workspace should exist. Changing this forces a new Healthcare Workspace to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the Healthcare Workspace.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct WorkspaceResult {
        /// Specifies the Azure Region where the Healthcare Workspace should be created. Changing this forces a new Healthcare Workspace to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Healthcare Workspace. Changing this forces a new Healthcare Workspace to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub private_endpoint_connections: pulumi_gestalt_rust::Output<
            Vec<super::super::types::healthcare::WorkspacePrivateEndpointConnection>,
        >,
        /// Specifies the name of the Resource Group where the Healthcare Workspace should exist. Changing this forces a new Healthcare Workspace to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the Healthcare Workspace.
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
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:healthcare/workspace:Workspace".into(),
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
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            private_endpoint_connections: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateEndpointConnections"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
