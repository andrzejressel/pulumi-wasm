pub mod get_access_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccessPolicyArgs {
        /// The parent of this AccessPolicy in the Cloud Resource Hierarchy. Format: `organizations/{{organization_id}}`
        #[builder(into)]
        pub parent: pulumi_wasm_rust::Output<String>,
        /// Folder or project on which this policy is applicable. Format: `folders/{{folder_id}}` or `projects/{{project_number}}`
        #[builder(into, default)]
        pub scopes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct GetAccessPolicyResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Resource name of the AccessPolicy.
        pub name: pulumi_wasm_rust::Output<String>,
        pub parent: pulumi_wasm_rust::Output<String>,
        pub scopes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Human readable title. Does not affect behavior.
        pub title: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAccessPolicyArgs) -> GetAccessPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let parent_binding = args.parent.get_inner();
        let scopes_binding = args.scopes.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:accesscontextmanager/getAccessPolicy:getAccessPolicy".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "scopes".into(),
                    value: &scopes_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "scopes".into(),
                },
                register_interface::ResultField {
                    name: "title".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAccessPolicyResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            scopes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scopes").unwrap(),
            ),
            title: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("title").unwrap(),
            ),
        }
    }
}
