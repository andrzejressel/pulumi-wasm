pub mod get_workspace_private_endpoint_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetWorkspacePrivateEndpointConnectionArgs {
        /// The resource ID of the Private Endpoint.
        #[builder(into)]
        pub private_endpoint_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The resource ID of the Databricks Workspace.
        #[builder(into)]
        pub workspace_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetWorkspacePrivateEndpointConnectionResult {
        /// A `connections` block as documented below.
        pub connections: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::databricks::GetWorkspacePrivateEndpointConnectionConnection,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the Private Endpoint.
        pub private_endpoint_id: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the Databricks Workspace.
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetWorkspacePrivateEndpointConnectionArgs,
    ) -> GetWorkspacePrivateEndpointConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let private_endpoint_id_binding = args
            .private_endpoint_id
            .get_output(context)
            .get_inner();
        let workspace_id_binding = args.workspace_id.get_output(context).get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "connections".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "privateEndpointId".into(),
                },
                register_interface::ResultField {
                    name: "workspaceId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetWorkspacePrivateEndpointConnectionResult {
            connections: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connections").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            private_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateEndpointId").unwrap(),
            ),
            workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceId").unwrap(),
            ),
        }
    }
}
