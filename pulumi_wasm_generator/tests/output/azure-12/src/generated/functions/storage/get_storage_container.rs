pub mod get_storage_container {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetStorageContainerArgs {
        /// A mapping of MetaData for this Container.
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Container.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The id of the Storage Account where the Container exists. This property will become Required in version 5.0 of the Provider.
        ///
        /// > **NOTE:** One of `storage_account_name` or `storage_account_id` must be specified. When specifying `storage_account_id` the resource will use the Resource Manager API, rather than the Data Plane API.
        #[builder(into, default)]
        pub storage_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Storage Account where the Container exists. This property is deprecated in favour of `storage_account_id`.
        #[builder(into, default)]
        pub storage_account_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetStorageContainerResult {
        /// The Access Level configured for this Container.
        pub container_access_type: pulumi_wasm_rust::Output<String>,
        /// The default encryption scope in use for blobs uploaded to this container.
        pub default_encryption_scope: pulumi_wasm_rust::Output<String>,
        /// Whether blobs are allowed to override the default encryption scope for this container.
        pub encryption_scope_override_enabled: pulumi_wasm_rust::Output<bool>,
        /// Is there an Immutability Policy configured on this Storage Container?
        pub has_immutability_policy: pulumi_wasm_rust::Output<bool>,
        /// Is there a Legal Hold configured on this Storage Container?
        pub has_legal_hold: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A mapping of MetaData for this Container.
        pub metadata: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_manager_id: pulumi_wasm_rust::Output<String>,
        pub storage_account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub storage_account_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetStorageContainerArgs) -> GetStorageContainerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let metadata_binding = args.metadata.get_inner();
        let name_binding = args.name.get_inner();
        let storage_account_id_binding = args.storage_account_id.get_inner();
        let storage_account_name_binding = args.storage_account_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:storage/getStorageContainer:getStorageContainer".into(),
            version: super::super::super::get_version(),
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
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountName".into(),
                    value: &storage_account_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "containerAccessType".into(),
                },
                register_interface::ResultField {
                    name: "defaultEncryptionScope".into(),
                },
                register_interface::ResultField {
                    name: "encryptionScopeOverrideEnabled".into(),
                },
                register_interface::ResultField {
                    name: "hasImmutabilityPolicy".into(),
                },
                register_interface::ResultField {
                    name: "hasLegalHold".into(),
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
                    name: "resourceManagerId".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountId".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetStorageContainerResult {
            container_access_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerAccessType").unwrap(),
            ),
            default_encryption_scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultEncryptionScope").unwrap(),
            ),
            encryption_scope_override_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionScopeOverrideEnabled").unwrap(),
            ),
            has_immutability_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hasImmutabilityPolicy").unwrap(),
            ),
            has_legal_hold: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hasLegalHold").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadata").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_manager_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceManagerId").unwrap(),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountId").unwrap(),
            ),
            storage_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountName").unwrap(),
            ),
        }
    }
}
