/// Provides an Amazon Managed Grafana workspace API Key resource.
///
/// ## Example Usage
///
/// ### Basic configuration
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let key = workspace_api_key::create(
///         "key",
///         WorkspaceApiKeyArgs::builder()
///             .key_name("test-key")
///             .key_role("VIEWER")
///             .seconds_to_live(3600)
///             .workspace_id("${test.id}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workspace_api_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceApiKeyArgs {
        /// Specifies the name of the API key. Key names must be unique to the workspace.
        #[builder(into)]
        pub key_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the permission level of the API key. Valid values are `VIEWER`, `EDITOR`, or `ADMIN`.
        #[builder(into)]
        pub key_role: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the time in seconds until the API key expires. Keys can be valid for up to 30 days.
        #[builder(into)]
        pub seconds_to_live: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The ID of the workspace that the API key is valid for.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkspaceApiKeyResult {
        /// The key token in JSON format. Use this value as a bearer token to authenticate HTTP requests to the workspace.
        pub key: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the API key. Key names must be unique to the workspace.
        pub key_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the permission level of the API key. Valid values are `VIEWER`, `EDITOR`, or `ADMIN`.
        pub key_role: pulumi_gestalt_rust::Output<String>,
        /// Specifies the time in seconds until the API key expires. Keys can be valid for up to 30 days.
        pub seconds_to_live: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the workspace that the API key is valid for.
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkspaceApiKeyArgs,
    ) -> WorkspaceApiKeyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let key_name_binding = args.key_name.get_output(context);
        let key_role_binding = args.key_role.get_output(context);
        let seconds_to_live_binding = args.seconds_to_live.get_output(context);
        let workspace_id_binding = args.workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:grafana/workspaceApiKey:WorkspaceApiKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyName".into(),
                    value: key_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyRole".into(),
                    value: key_role_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secondsToLive".into(),
                    value: seconds_to_live_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceId".into(),
                    value: workspace_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkspaceApiKeyResult {
            key: o.get_field("key"),
            key_name: o.get_field("keyName"),
            key_role: o.get_field("keyRole"),
            seconds_to_live: o.get_field("secondsToLive"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}
