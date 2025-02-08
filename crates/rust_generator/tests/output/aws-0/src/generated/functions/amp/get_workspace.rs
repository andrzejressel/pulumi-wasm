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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetWorkspaceArgs,
    ) -> GetWorkspaceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let tags_binding = args.tags.get_output(context).get_inner();
        let workspace_id_binding = args.workspace_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:amp/getWorkspace:getWorkspace".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetWorkspaceResult {
            alias: pulumi_gestalt_rust::__private::into_domain(o.extract_field("alias")),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            created_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdDate"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            kms_key_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyArn"),
            ),
            prometheus_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("prometheusEndpoint"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            workspace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workspaceId"),
            ),
        }
    }
}
