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
///     let exampleWorkspaceServiceAccountToken = workspace_service_account_token::create(
///         "exampleWorkspaceServiceAccountToken",
///         WorkspaceServiceAccountTokenArgs::builder()
///             .name("example-key")
///             .seconds_to_live(3600)
///             .service_account_id("${example.serviceAccountId}")
///             .workspace_id("${exampleAwsGrafanaWorkspace.id}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workspace_service_account_token {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceServiceAccountTokenArgs {
        /// A name for the token to create. The name must be unique within the workspace.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Sets how long the token will be valid, in seconds. You can set the time up to 30 days in the future.
        #[builder(into)]
        pub seconds_to_live: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The ID of the service account for which to create a token.
        #[builder(into)]
        pub service_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Grafana workspace with which the service account token is associated.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkspaceServiceAccountTokenResult {
        /// Specifies when the service account token was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// Specifies when the service account token will expire.
        pub expires_at: pulumi_gestalt_rust::Output<String>,
        /// The key for the service account token. Used when making calls to the Grafana HTTP APIs to authenticate and authorize the requests.
        pub key: pulumi_gestalt_rust::Output<String>,
        /// A name for the token to create. The name must be unique within the workspace.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Sets how long the token will be valid, in seconds. You can set the time up to 30 days in the future.
        pub seconds_to_live: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the service account for which to create a token.
        pub service_account_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the service account token in the given Grafana workspace.
        pub service_account_token_id: pulumi_gestalt_rust::Output<String>,
        /// The Grafana workspace with which the service account token is associated.
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkspaceServiceAccountTokenArgs,
    ) -> WorkspaceServiceAccountTokenResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let seconds_to_live_binding = args.seconds_to_live.get_output(context);
        let service_account_id_binding = args.service_account_id.get_output(context);
        let workspace_id_binding = args.workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:grafana/workspaceServiceAccountToken:WorkspaceServiceAccountToken"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secondsToLive".into(),
                    value: &seconds_to_live_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceAccountId".into(),
                    value: &service_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkspaceServiceAccountTokenResult {
            created_at: o.get_field("createdAt"),
            expires_at: o.get_field("expiresAt"),
            key: o.get_field("key"),
            name: o.get_field("name"),
            seconds_to_live: o.get_field("secondsToLive"),
            service_account_id: o.get_field("serviceAccountId"),
            service_account_token_id: o.get_field("serviceAccountTokenId"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}
