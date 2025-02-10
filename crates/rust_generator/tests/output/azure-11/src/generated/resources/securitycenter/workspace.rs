/// Manages the subscription's Security Center Workspace.
///
/// > **NOTE:** Owner access permission is required.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("tfex-security-workspace")
///             .build_struct(),
///     );
///     let exampleAnalyticsWorkspace = analytics_workspace::create(
///         "exampleAnalyticsWorkspace",
///         AnalyticsWorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("tfex-security-workspace")
///             .resource_group_name("${example.name}")
///             .sku("PerGB2018")
///             .build_struct(),
///     );
///     let exampleWorkspace = workspace::create(
///         "exampleWorkspace",
///         WorkspaceArgs::builder()
///             .scope("/subscriptions/00000000-0000-0000-0000-000000000000")
///             .workspace_id("${exampleAnalyticsWorkspace.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// The contact can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:securitycenter/workspace:Workspace example /subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.Security/workspaceSettings/default
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workspace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceArgs {
        /// The scope of VMs to send their security data to the desired workspace, unless overridden by a setting with more specific scope.
        #[builder(into)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Log Analytics Workspace to save the data in.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkspaceResult {
        /// The scope of VMs to send their security data to the desired workspace, unless overridden by a setting with more specific scope.
        pub scope: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Log Analytics Workspace to save the data in.
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkspaceArgs,
    ) -> WorkspaceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let scope_binding = args.scope.get_output(context);
        let workspace_id_binding = args.workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:securitycenter/workspace:Workspace".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: scope_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceId".into(),
                    value: workspace_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkspaceResult {
            scope: o.get_field("scope"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}
