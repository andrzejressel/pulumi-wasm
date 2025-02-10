#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_workspace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetWorkspaceArgs {
        /// Tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Prometheus workspace ID.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetWorkspaceResult {
        /// Prometheus workspace alias.
        pub alias: pulumi_gestalt_rust::Output<String>,
        /// ARN of the Prometheus workspace.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Creation date of the Prometheus workspace.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the KMS key used to encrypt data in the Prometheus workspace.
        pub kms_key_arn: pulumi_gestalt_rust::Output<String>,
        /// Endpoint of the Prometheus workspace.
        pub prometheus_endpoint: pulumi_gestalt_rust::Output<String>,
        /// Status of the Prometheus workspace.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetWorkspaceArgs,
    ) -> GetWorkspaceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let tags_binding = args.tags.get_output(context);
        let workspace_id_binding = args.workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:amp/getWorkspace:getWorkspace".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceId".into(),
                    value: workspace_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetWorkspaceResult {
            alias: o.get_field("alias"),
            arn: o.get_field("arn"),
            created_date: o.get_field("createdDate"),
            id: o.get_field("id"),
            kms_key_arn: o.get_field("kmsKeyArn"),
            prometheus_endpoint: o.get_field("prometheusEndpoint"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}
