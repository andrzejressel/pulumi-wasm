pub mod get_project {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProjectArgs {
        /// The project ID. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetProjectResult {
        pub auto_create_network: pulumi_wasm_rust::Output<bool>,
        pub billing_account: pulumi_wasm_rust::Output<String>,
        pub deletion_policy: pulumi_wasm_rust::Output<String>,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub folder_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The numeric identifier of the project.
        pub number: pulumi_wasm_rust::Output<String>,
        pub org_id: pulumi_wasm_rust::Output<String>,
        pub project_id: pulumi_wasm_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetProjectArgs) -> GetProjectResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let project_id_binding = args.project_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:organizations/getProject:getProject".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "projectId".into(),
                    value: &project_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "autoCreateNetwork".into(),
                },
                register_interface::ResultField {
                    name: "billingAccount".into(),
                },
                register_interface::ResultField {
                    name: "deletionPolicy".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "folderId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "number".into(),
                },
                register_interface::ResultField {
                    name: "orgId".into(),
                },
                register_interface::ResultField {
                    name: "projectId".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetProjectResult {
            auto_create_network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoCreateNetwork").unwrap(),
            ),
            billing_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("billingAccount").unwrap(),
            ),
            deletion_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionPolicy").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            folder_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("folderId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("number").unwrap(),
            ),
            org_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("orgId").unwrap(),
            ),
            project_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("projectId").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
