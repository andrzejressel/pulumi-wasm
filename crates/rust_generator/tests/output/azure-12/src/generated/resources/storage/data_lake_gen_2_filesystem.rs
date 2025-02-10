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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod data_lake_gen_2_filesystem {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataLakeGen2FilesystemArgs {
        /// One or more `ace` blocks as defined below to specify the entries for the ACL for the path.
        #[builder(into, default)]
        pub aces: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::storage::DataLakeGen2FilesystemAce>>,
        >,
        /// The default encryption scope to use for this filesystem. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub default_encryption_scope: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Object ID of the Azure Active Directory Group to make the owning group of the root path (i.e. `/`). Possible values also include `$superuser`.
        ///
        /// > **NOTE:** The Storage Account requires `account_kind` to be either `StorageV2` or `BlobStorage`. In addition, `is_hns_enabled` has to be set to `true`.
        #[builder(into, default)]
        pub group: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Data Lake Gen2 File System which should be created within the Storage Account. Must be unique within the storage account the queue is located. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Object ID of the Azure Active Directory User to make the owning user of the root path (i.e. `/`). Possible values also include `$superuser`.
        #[builder(into, default)]
        pub owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of Key to Base64-Encoded Values which should be assigned to this Data Lake Gen2 File System.
        #[builder(into, default)]
        pub properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the ID of the Storage Account in which the Data Lake Gen2 File System should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DataLakeGen2FilesystemResult {
        /// One or more `ace` blocks as defined below to specify the entries for the ACL for the path.
        pub aces: pulumi_gestalt_rust::Output<
            Vec<super::super::types::storage::DataLakeGen2FilesystemAce>,
        >,
        /// The default encryption scope to use for this filesystem. Changing this forces a new resource to be created.
        pub default_encryption_scope: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Object ID of the Azure Active Directory Group to make the owning group of the root path (i.e. `/`). Possible values also include `$superuser`.
        ///
        /// > **NOTE:** The Storage Account requires `account_kind` to be either `StorageV2` or `BlobStorage`. In addition, `is_hns_enabled` has to be set to `true`.
        pub group: pulumi_gestalt_rust::Output<String>,
        /// The name of the Data Lake Gen2 File System which should be created within the Storage Account. Must be unique within the storage account the queue is located. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Object ID of the Azure Active Directory User to make the owning user of the root path (i.e. `/`). Possible values also include `$superuser`.
        pub owner: pulumi_gestalt_rust::Output<String>,
        /// A mapping of Key to Base64-Encoded Values which should be assigned to this Data Lake Gen2 File System.
        pub properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the ID of the Storage Account in which the Data Lake Gen2 File System should exist. Changing this forces a new resource to be created.
        pub storage_account_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataLakeGen2FilesystemArgs,
    ) -> DataLakeGen2FilesystemResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let aces_binding = args.aces.get_output(context);
        let default_encryption_scope_binding = args
            .default_encryption_scope
            .get_output(context);
        let group_binding = args.group.get_output(context);
        let name_binding = args.name.get_output(context);
        let owner_binding = args.owner.get_output(context);
        let properties_binding = args.properties.get_output(context);
        let storage_account_id_binding = args.storage_account_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:storage/dataLakeGen2Filesystem:DataLakeGen2Filesystem".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "aces".into(),
                    value: aces_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultEncryptionScope".into(),
                    value: default_encryption_scope_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "group".into(),
                    value: group_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "owner".into(),
                    value: owner_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "properties".into(),
                    value: properties_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountId".into(),
                    value: storage_account_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DataLakeGen2FilesystemResult {
            aces: o.get_field("aces"),
            default_encryption_scope: o.get_field("defaultEncryptionScope"),
            group: o.get_field("group"),
            name: o.get_field("name"),
            owner: o.get_field("owner"),
            properties: o.get_field("properties"),
            storage_account_id: o.get_field("storageAccountId"),
        }
    }
}
