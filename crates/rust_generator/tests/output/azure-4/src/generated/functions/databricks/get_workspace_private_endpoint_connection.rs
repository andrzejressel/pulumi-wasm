#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_workspace_private_endpoint_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetWorkspacePrivateEndpointConnectionArgs {
        /// The resource ID of the Private Endpoint.
        #[builder(into)]
        pub private_endpoint_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource ID of the Databricks Workspace.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetWorkspacePrivateEndpointConnectionResult {
        /// A `connections` block as documented below.
        pub connections: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::databricks::GetWorkspacePrivateEndpointConnectionConnection,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The resource ID of the Private Endpoint.
        pub private_endpoint_id: pulumi_gestalt_rust::Output<String>,
        /// The resource ID of the Databricks Workspace.
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetWorkspacePrivateEndpointConnectionArgs,
    ) -> GetWorkspacePrivateEndpointConnectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let private_endpoint_id_binding_1 = args.private_endpoint_id.get_output(context);
        let private_endpoint_id_binding = private_endpoint_id_binding_1.get_inner();
        let workspace_id_binding_1 = args.workspace_id.get_output(context);
        let workspace_id_binding = workspace_id_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:databricks/getWorkspacePrivateEndpointConnection:getWorkspacePrivateEndpointConnection"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "privateEndpointId".into(),
                    value: &private_endpoint_id_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetWorkspacePrivateEndpointConnectionResult {
            connections: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("connections"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            private_endpoint_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateEndpointId"),
            ),
            workspace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workspaceId"),
            ),
        }
    }
}
