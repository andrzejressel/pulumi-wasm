pub mod zip_blob {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZipBlobArgs {
        #[builder(into, default)]
        pub access_tier: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub cache_control: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub content: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub content_md5: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub content_type: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub encryption_scope: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub parallelism: pulumi_wasm_rust::Output<Option<i32>>,
        #[builder(into, default)]
        pub size: pulumi_wasm_rust::Output<Option<i32>>,
        #[builder(into, default)]
        pub source_content: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub source_uri: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into)]
        pub storage_account_name: pulumi_wasm_rust::Output<String>,
        #[builder(into)]
        pub storage_container_name: pulumi_wasm_rust::Output<String>,
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ZipBlobResult {
        pub access_tier: pulumi_wasm_rust::Output<String>,
        pub cache_control: pulumi_wasm_rust::Output<Option<String>>,
        pub content: pulumi_wasm_rust::Output<Option<String>>,
        pub content_md5: pulumi_wasm_rust::Output<Option<String>>,
        pub content_type: pulumi_wasm_rust::Output<Option<String>>,
        pub encryption_scope: pulumi_wasm_rust::Output<Option<String>>,
        pub metadata: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub parallelism: pulumi_wasm_rust::Output<Option<i32>>,
        pub size: pulumi_wasm_rust::Output<Option<i32>>,
        pub source_content: pulumi_wasm_rust::Output<Option<String>>,
        pub source_uri: pulumi_wasm_rust::Output<Option<String>>,
        pub storage_account_name: pulumi_wasm_rust::Output<String>,
        pub storage_container_name: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ZipBlobArgs) -> ZipBlobResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_tier_binding = args.access_tier.get_inner();
        let cache_control_binding = args.cache_control.get_inner();
        let content_binding = args.content.get_inner();
        let content_md5_binding = args.content_md5.get_inner();
        let content_type_binding = args.content_type.get_inner();
        let encryption_scope_binding = args.encryption_scope.get_inner();
        let metadata_binding = args.metadata.get_inner();
        let name_binding = args.name.get_inner();
        let parallelism_binding = args.parallelism.get_inner();
        let size_binding = args.size.get_inner();
        let source_content_binding = args.source_content.get_inner();
        let source_uri_binding = args.source_uri.get_inner();
        let storage_account_name_binding = args.storage_account_name.get_inner();
        let storage_container_name_binding = args.storage_container_name.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/zipBlob:ZipBlob".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessTier".into(),
                    value: &access_tier_binding,
                },
                register_interface::ObjectField {
                    name: "cacheControl".into(),
                    value: &cache_control_binding,
                },
                register_interface::ObjectField {
                    name: "content".into(),
                    value: &content_binding,
                },
                register_interface::ObjectField {
                    name: "contentMd5".into(),
                    value: &content_md5_binding,
                },
                register_interface::ObjectField {
                    name: "contentType".into(),
                    value: &content_type_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionScope".into(),
                    value: &encryption_scope_binding,
                },
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parallelism".into(),
                    value: &parallelism_binding,
                },
                register_interface::ObjectField {
                    name: "size".into(),
                    value: &size_binding,
                },
                register_interface::ObjectField {
                    name: "sourceContent".into(),
                    value: &source_content_binding,
                },
                register_interface::ObjectField {
                    name: "sourceUri".into(),
                    value: &source_uri_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountName".into(),
                    value: &storage_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "storageContainerName".into(),
                    value: &storage_container_name_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessTier".into(),
                },
                register_interface::ResultField {
                    name: "cacheControl".into(),
                },
                register_interface::ResultField {
                    name: "content".into(),
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
                    name: "metadata".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parallelism".into(),
                },
                register_interface::ResultField {
                    name: "size".into(),
                },
                register_interface::ResultField {
                    name: "sourceContent".into(),
                },
                register_interface::ResultField {
                    name: "sourceUri".into(),
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
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ZipBlobResult {
            access_tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessTier").unwrap(),
            ),
            cache_control: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cacheControl").unwrap(),
            ),
            content: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("content").unwrap(),
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
            metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadata").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parallelism: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parallelism").unwrap(),
            ),
            size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("size").unwrap(),
            ),
            source_content: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceContent").unwrap(),
            ),
            source_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceUri").unwrap(),
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
