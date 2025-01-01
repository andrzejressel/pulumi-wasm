pub mod get_active_folder {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetActiveFolderArgs {
        /// The API method to use to search for the folder. Valid values are `LIST` and `SEARCH`. Default Value is `LIST`. `LIST` is [strongly consistent](https://cloud.google.com/resource-manager/reference/rest/v3/folders/list#:~:text=list()%20provides%20a-,strongly%20consistent,-view%20of%20the) and requires `resourcemanager.folders.list` on the parent folder, while `SEARCH` is [eventually consistent](https://cloud.google.com/resource-manager/reference/rest/v3/folders/search#:~:text=eventually%20consistent) and only returns folders that the user has `resourcemanager.folders.get` permission on.
        #[builder(into, default)]
        pub api_method: pulumi_wasm_rust::Output<Option<String>>,
        /// The folder's display name.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The resource name of the parent Folder or Organization.
        #[builder(into)]
        pub parent: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetActiveFolderResult {
        pub api_method: pulumi_wasm_rust::Output<Option<String>>,
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The resource name of the Folder. This uniquely identifies the folder.
        pub name: pulumi_wasm_rust::Output<String>,
        pub parent: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetActiveFolderArgs) -> GetActiveFolderResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_method_binding = args.api_method.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let parent_binding = args.parent.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:organizations/getActiveFolder:getActiveFolder".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiMethod".into(),
                    value: &api_method_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiMethod".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetActiveFolderResult {
            api_method: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiMethod").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
        }
    }
}
