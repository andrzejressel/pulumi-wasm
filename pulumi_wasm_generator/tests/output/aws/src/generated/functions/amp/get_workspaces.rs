pub mod get_workspaces {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetWorkspacesArgs {
        /// Limits results to workspaces with aliases that begin with this value.
        #[builder(into, default)]
        pub alias_prefix: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetWorkspacesResult {
        pub alias_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// List of aliases of the matched Prometheus workspaces.
        pub aliases: pulumi_wasm_rust::Output<Vec<String>>,
        /// List of ARNs of the matched Prometheus workspaces.
        pub arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// List of workspace IDs of the matched Prometheus workspaces.
        pub workspace_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetWorkspacesArgs) -> GetWorkspacesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alias_prefix_binding = args.alias_prefix.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:amp/getWorkspaces:getWorkspaces".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "aliasPrefix".into(),
                    value: &alias_prefix_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "aliasPrefix".into(),
                },
                register_interface::ResultField {
                    name: "aliases".into(),
                },
                register_interface::ResultField {
                    name: "arns".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "workspaceIds".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetWorkspacesResult {
            alias_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("aliasPrefix").unwrap(),
            ),
            aliases: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("aliases").unwrap(),
            ),
            arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arns").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            workspace_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceIds").unwrap(),
            ),
        }
    }
}
