pub mod get_folders {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFoldersArgs {
        /// A string parent as defined in the [REST API](https://cloud.google.com/resource-manager/reference/rest/v3/folders/list#query-parameters).
        #[builder(into)]
        pub parent_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetFoldersResult {
        /// A list of folders matching the provided filter. Structure is defined below.
        pub folders: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::organizations::GetFoldersFolder>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub parent_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetFoldersArgs) -> GetFoldersResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let parent_id_binding = args.parent_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:organizations/getFolders:getFolders".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "parentId".into(),
                    value: &parent_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "folders".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "parentId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetFoldersResult {
            folders: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("folders").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            parent_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parentId").unwrap(),
            ),
        }
    }
}
