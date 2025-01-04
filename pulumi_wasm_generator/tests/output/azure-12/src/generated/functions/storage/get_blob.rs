pub mod get_blob {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBlobArgs {
        /// A map of custom blob metadata.
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Blob.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Storage Account where the Container exists.
        #[builder(into)]
        pub storage_account_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Storage Container where the Blob exists.
        #[builder(into)]
        pub storage_container_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetBlobResult {
        /// The access tier of the storage blob.
        pub access_tier: pulumi_wasm_rust::Output<String>,
        /// The MD5 sum of the blob contents.
        pub content_md5: pulumi_wasm_rust::Output<String>,
        /// The content type of the storage blob.
        pub content_type: pulumi_wasm_rust::Output<String>,
        /// The encryption scope for this blob.
        pub encryption_scope: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A map of custom blob metadata.
        pub metadata: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub storage_account_name: pulumi_wasm_rust::Output<String>,
        pub storage_container_name: pulumi_wasm_rust::Output<String>,
        /// The type of the storage blob
        pub type_: pulumi_wasm_rust::Output<String>,
        /// The URL of the storage blob.
        pub url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetBlobArgs) -> GetBlobResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let metadata_binding = args.metadata.get_inner();
        let name_binding = args.name.get_inner();
        let storage_account_name_binding = args.storage_account_name.get_inner();
        let storage_container_name_binding = args.storage_container_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:storage/getBlob:getBlob".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountName".into(),
                    value: &storage_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "storageContainerName".into(),
                    value: &storage_container_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessTier".into(),
                },
                register_interface::ResultField {
                    name: "contentMd5".into(),
                },
                register_interface::ResultField {
                    name: "contentType".into(),
                },
                register_interface::ResultField {
                    name: "encryptionScope".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "metadata".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountName".into(),
                },
                register_interface::ResultField {
                    name: "storageContainerName".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "url".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetBlobResult {
            access_tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessTier").unwrap(),
            ),
            content_md5: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentMd5").unwrap(),
            ),
            content_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentType").unwrap(),
            ),
            encryption_scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionScope").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadata").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            storage_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountName").unwrap(),
            ),
            storage_container_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageContainerName").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            url: pulumi_wasm_rust::__private::into_domain(hashmap.remove("url").unwrap()),
        }
    }
}
