/// Manages a Data Lake Gen2 File System within an Azure Storage Account.
///
/// > **NOTE:** This resource requires some `Storage` specific roles which are not granted by default. Some of the built-ins roles that can be attributed are [`Storage Account Contributor`](https://docs.microsoft.com/azure/role-based-access-control/built-in-roles#storage-account-contributor), [`Storage Blob Data Owner`](https://docs.microsoft.com/azure/role-based-access-control/built-in-roles#storage-blob-data-owner), [`Storage Blob Data Contributor`](https://docs.microsoft.com/azure/role-based-access-control/built-in-roles#storage-blob-data-contributor), [`Storage Blob Data Reader`](https://docs.microsoft.com/azure/role-based-access-control/built-in-roles#storage-blob-data-reader).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplestorageacc
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///       accountKind: StorageV2
///       isHnsEnabled: 'true'
///   exampleDataLakeGen2Filesystem:
///     type: azure:storage:DataLakeGen2Filesystem
///     name: example
///     properties:
///       name: example
///       storageAccountId: ${exampleAccount.id}
///       properties:
///         hello: aGVsbG8=
/// ```
///
/// ## Import
///
/// Data Lake Gen2 File System's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/dataLakeGen2Filesystem:DataLakeGen2Filesystem queue1 https://account1.dfs.core.windows.net/fileSystem1
/// ```
///
pub mod data_lake_gen_2_filesystem {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataLakeGen2FilesystemArgs {
        /// One or more `ace` blocks as defined below to specify the entries for the ACL for the path.
        #[builder(into, default)]
        pub aces: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::storage::DataLakeGen2FilesystemAce>>,
        >,
        /// The default encryption scope to use for this filesystem. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub default_encryption_scope: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Object ID of the Azure Active Directory Group to make the owning group of the root path (i.e. `/`). Possible values also include `$superuser`.
        ///
        /// > **NOTE:** The Storage Account requires `account_kind` to be either `StorageV2` or `BlobStorage`. In addition, `is_hns_enabled` has to be set to `true`.
        #[builder(into, default)]
        pub group: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Data Lake Gen2 File System which should be created within the Storage Account. Must be unique within the storage account the queue is located. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Object ID of the Azure Active Directory User to make the owning user of the root path (i.e. `/`). Possible values also include `$superuser`.
        #[builder(into, default)]
        pub owner: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of Key to Base64-Encoded Values which should be assigned to this Data Lake Gen2 File System.
        #[builder(into, default)]
        pub properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the ID of the Storage Account in which the Data Lake Gen2 File System should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DataLakeGen2FilesystemResult {
        /// One or more `ace` blocks as defined below to specify the entries for the ACL for the path.
        pub aces: pulumi_wasm_rust::Output<
            Vec<super::super::types::storage::DataLakeGen2FilesystemAce>,
        >,
        /// The default encryption scope to use for this filesystem. Changing this forces a new resource to be created.
        pub default_encryption_scope: pulumi_wasm_rust::Output<String>,
        /// Specifies the Object ID of the Azure Active Directory Group to make the owning group of the root path (i.e. `/`). Possible values also include `$superuser`.
        ///
        /// > **NOTE:** The Storage Account requires `account_kind` to be either `StorageV2` or `BlobStorage`. In addition, `is_hns_enabled` has to be set to `true`.
        pub group: pulumi_wasm_rust::Output<String>,
        /// The name of the Data Lake Gen2 File System which should be created within the Storage Account. Must be unique within the storage account the queue is located. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the Object ID of the Azure Active Directory User to make the owning user of the root path (i.e. `/`). Possible values also include `$superuser`.
        pub owner: pulumi_wasm_rust::Output<String>,
        /// A mapping of Key to Base64-Encoded Values which should be assigned to this Data Lake Gen2 File System.
        pub properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the ID of the Storage Account in which the Data Lake Gen2 File System should exist. Changing this forces a new resource to be created.
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DataLakeGen2FilesystemArgs,
    ) -> DataLakeGen2FilesystemResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aces_binding = args.aces.get_inner();
        let default_encryption_scope_binding = args.default_encryption_scope.get_inner();
        let group_binding = args.group.get_inner();
        let name_binding = args.name.get_inner();
        let owner_binding = args.owner.get_inner();
        let properties_binding = args.properties.get_inner();
        let storage_account_id_binding = args.storage_account_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/dataLakeGen2Filesystem:DataLakeGen2Filesystem".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "aces".into(),
                    value: &aces_binding,
                },
                register_interface::ObjectField {
                    name: "defaultEncryptionScope".into(),
                    value: &default_encryption_scope_binding,
                },
                register_interface::ObjectField {
                    name: "group".into(),
                    value: &group_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "owner".into(),
                    value: &owner_binding,
                },
                register_interface::ObjectField {
                    name: "properties".into(),
                    value: &properties_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "aces".into(),
                },
                register_interface::ResultField {
                    name: "defaultEncryptionScope".into(),
                },
                register_interface::ResultField {
                    name: "group".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "owner".into(),
                },
                register_interface::ResultField {
                    name: "properties".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DataLakeGen2FilesystemResult {
            aces: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("aces").unwrap(),
            ),
            default_encryption_scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultEncryptionScope").unwrap(),
            ),
            group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("group").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owner").unwrap(),
            ),
            properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("properties").unwrap(),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountId").unwrap(),
            ),
        }
    }
}