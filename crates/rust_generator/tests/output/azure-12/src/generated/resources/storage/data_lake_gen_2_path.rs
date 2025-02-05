/// Manages a Data Lake Gen2 Path in a File System within an Azure Storage Account.
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
///   exampleDataLakeGen2Path:
///     type: azure:storage:DataLakeGen2Path
///     name: example
///     properties:
///       path: example
///       filesystemName: ${exampleDataLakeGen2Filesystem.name}
///       storageAccountId: ${exampleAccount.id}
///       resource: directory
/// ```
///
/// ## Import
///
/// Data Lake Gen2 Paths can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/dataLakeGen2Path:DataLakeGen2Path example https://account1.dfs.core.windows.net/fileSystem1/path
/// ```
///
pub mod data_lake_gen_2_path {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataLakeGen2PathArgs {
        /// One or more `ace` blocks as defined below to specify the entries for the ACL for the path.
        #[builder(into, default)]
        pub aces: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::storage::DataLakeGen2PathAce>>,
        >,
        /// The name of the Data Lake Gen2 File System which should be created within the Storage Account. Must be unique within the storage account the queue is located. Changing this forces a new resource to be created.
        #[builder(into)]
        pub filesystem_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the Object ID of the Azure Active Directory Group to make the owning group. Possible values also include `$superuser`.
        #[builder(into, default)]
        pub group: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the Object ID of the Azure Active Directory User to make the owning user. Possible values also include `$superuser`.
        #[builder(into, default)]
        pub owner: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The path which should be created within the Data Lake Gen2 File System in the Storage Account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub path: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the type for path to create. Currently only `directory` is supported. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the ID of the Storage Account in which the Data Lake Gen2 File System should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_account_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DataLakeGen2PathResult {
        /// One or more `ace` blocks as defined below to specify the entries for the ACL for the path.
        pub aces: pulumi_wasm_rust::Output<
            Vec<super::super::types::storage::DataLakeGen2PathAce>,
        >,
        /// The name of the Data Lake Gen2 File System which should be created within the Storage Account. Must be unique within the storage account the queue is located. Changing this forces a new resource to be created.
        pub filesystem_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the Object ID of the Azure Active Directory Group to make the owning group. Possible values also include `$superuser`.
        pub group: pulumi_wasm_rust::Output<String>,
        /// Specifies the Object ID of the Azure Active Directory User to make the owning user. Possible values also include `$superuser`.
        pub owner: pulumi_wasm_rust::Output<String>,
        /// The path which should be created within the Data Lake Gen2 File System in the Storage Account. Changing this forces a new resource to be created.
        pub path: pulumi_wasm_rust::Output<String>,
        /// Specifies the type for path to create. Currently only `directory` is supported. Changing this forces a new resource to be created.
        pub resource: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the Storage Account in which the Data Lake Gen2 File System should exist. Changing this forces a new resource to be created.
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DataLakeGen2PathArgs,
    ) -> DataLakeGen2PathResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aces_binding = args.aces.get_output(context).get_inner();
        let filesystem_name_binding = args
            .filesystem_name
            .get_output(context)
            .get_inner();
        let group_binding = args.group.get_output(context).get_inner();
        let owner_binding = args.owner.get_output(context).get_inner();
        let path_binding = args.path.get_output(context).get_inner();
        let resource_binding = args.resource.get_output(context).get_inner();
        let storage_account_id_binding = args
            .storage_account_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/dataLakeGen2Path:DataLakeGen2Path".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "aces".into(),
                    value: &aces_binding,
                },
                register_interface::ObjectField {
                    name: "filesystemName".into(),
                    value: &filesystem_name_binding,
                },
                register_interface::ObjectField {
                    name: "group".into(),
                    value: &group_binding,
                },
                register_interface::ObjectField {
                    name: "owner".into(),
                    value: &owner_binding,
                },
                register_interface::ObjectField {
                    name: "path".into(),
                    value: &path_binding,
                },
                register_interface::ObjectField {
                    name: "resource".into(),
                    value: &resource_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DataLakeGen2PathResult {
            aces: pulumi_wasm_rust::__private::into_domain(o.extract_field("aces")),
            filesystem_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("filesystemName"),
            ),
            group: pulumi_wasm_rust::__private::into_domain(o.extract_field("group")),
            owner: pulumi_wasm_rust::__private::into_domain(o.extract_field("owner")),
            path: pulumi_wasm_rust::__private::into_domain(o.extract_field("path")),
            resource: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resource"),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageAccountId"),
            ),
        }
    }
}
