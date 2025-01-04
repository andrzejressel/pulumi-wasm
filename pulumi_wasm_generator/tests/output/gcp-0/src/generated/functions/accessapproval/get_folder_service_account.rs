pub mod get_folder_service_account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFolderServiceAccountArgs {
        /// The folder ID the service account was created for.
        #[builder(into)]
        pub folder_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetFolderServiceAccountResult {
        /// The email address of the service account. This value is
        /// often used to refer to the service account in order to grant IAM permissions.
        pub account_email: pulumi_wasm_rust::Output<String>,
        pub folder_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Access Approval service account resource name. Format is "folders/{folder_id}/serviceAccount".
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetFolderServiceAccountArgs) -> GetFolderServiceAccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let folder_id_binding = args.folder_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:accessapproval/getFolderServiceAccount:getFolderServiceAccount"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "folderId".into(),
                    value: &folder_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountEmail".into(),
                },
                register_interface::ResultField {
                    name: "folderId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetFolderServiceAccountResult {
            account_email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountEmail").unwrap(),
            ),
            folder_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("folderId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
