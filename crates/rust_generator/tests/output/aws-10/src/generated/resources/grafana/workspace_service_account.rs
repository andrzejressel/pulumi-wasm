/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = workspace_service_account::create(
///         "example",
///         WorkspaceServiceAccountArgs::builder()
///             .grafana_role("ADMIN")
///             .name("example-admin")
///             .workspace_id("${exampleAwsGrafanaWorkspace.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Managed Grafana Workspace Service Account using the `workspace_id` and `service_account_id` separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:grafana/workspaceServiceAccount:WorkspaceServiceAccount example g-abc12345,1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workspace_service_account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceServiceAccountArgs {
        /// The permission level to use for this service account. For more information about the roles and the permissions each has, see the [User roles](https://docs.aws.amazon.com/grafana/latest/userguide/Grafana-user-roles.html) documentation.
        #[builder(into)]
        pub grafana_role: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A name for the service account. The name must be unique within the workspace, as it determines the ID associated with the service account.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Grafana workspace with which the service account is associated.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkspaceServiceAccountResult {
        /// The permission level to use for this service account. For more information about the roles and the permissions each has, see the [User roles](https://docs.aws.amazon.com/grafana/latest/userguide/Grafana-user-roles.html) documentation.
        pub grafana_role: pulumi_gestalt_rust::Output<String>,
        /// A name for the service account. The name must be unique within the workspace, as it determines the ID associated with the service account.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the service account in the given Grafana workspace
        pub service_account_id: pulumi_gestalt_rust::Output<String>,
        /// The Grafana workspace with which the service account is associated.
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkspaceServiceAccountArgs,
    ) -> WorkspaceServiceAccountResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let grafana_role_binding = args.grafana_role.get_output(context);
        let name_binding = args.name.get_output(context);
        let workspace_id_binding = args.workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:grafana/workspaceServiceAccount:WorkspaceServiceAccount".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "grafanaRole".into(),
                    value: grafana_role_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceId".into(),
                    value: workspace_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkspaceServiceAccountResult {
            grafana_role: o.get_field("grafanaRole"),
            name: o.get_field("name"),
            service_account_id: o.get_field("serviceAccountId"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}
