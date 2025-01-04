/// Manages a Data Share Data Lake Gen2 Dataset.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAccount:
///     type: azure:datashare:Account
///     name: example
///     properties:
///       name: example-dsa
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       identity:
///         type: SystemAssigned
///   exampleShare:
///     type: azure:datashare:Share
///     name: example
///     properties:
///       name: example_ds
///       accountId: ${exampleAccount.id}
///       kind: CopyBased
///   exampleAccount2:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplestr
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       accountKind: BlobStorage
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleDataLakeGen2Filesystem:
///     type: azure:storage:DataLakeGen2Filesystem
///     name: example
///     properties:
///       name: example-dlg2fs
///       storageAccountId: ${exampleAccount2.id}
///   exampleAssignment:
///     type: azure:authorization:Assignment
///     name: example
///     properties:
///       scope: ${exampleAccount2.id}
///       roleDefinitionName: Storage Blob Data Reader
///       principalId: ${example.objectId}
///   exampleDatasetDataLakeGen2:
///     type: azure:datashare:DatasetDataLakeGen2
///     name: example
///     properties:
///       name: accexample-dlg2ds
///       shareId: ${exampleShare.id}
///       storageAccountId: ${exampleAccount2.id}
///       fileSystemName: ${exampleDataLakeGen2Filesystem.name}
///       filePath: myfile.txt
///     options:
///       dependsOn:
///         - ${exampleAssignment}
/// variables:
///   example:
///     fn::invoke:
///       function: azuread:getServicePrincipal
///       arguments:
///         displayName: ${exampleAccount.name}
/// ```
///
/// ## Import
///
/// Data Share Data Lake Gen2 Datasets can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datashare/datasetDataLakeGen2:DatasetDataLakeGen2 example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DataShare/accounts/account1/shares/share1/dataSets/dataSet1
/// ```
///
pub mod dataset_data_lake_gen_2 {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatasetDataLakeGen2Args {
        /// The path of the file in the data lake file system to be shared with the receiver. Conflicts with `folder_path` Changing this forces a new Data Share Data Lake Gen2 Dataset to be created.
        #[builder(into, default)]
        pub file_path: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the data lake file system to be shared with the receiver. Changing this forces a new Data Share Data Lake Gen2 Dataset to be created.
        #[builder(into)]
        pub file_system_name: pulumi_wasm_rust::Output<String>,
        /// The folder path in the data lake file system to be shared with the receiver. Conflicts with `file_path` Changing this forces a new Data Share Data Lake Gen2 Dataset to be created.
        #[builder(into, default)]
        pub folder_path: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Data Share Data Lake Gen2 Dataset. Changing this forces a new Data Share Data Lake Gen2 Dataset to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource ID of the Data Share where this Data Share Data Lake Gen2 Dataset should be created. Changing this forces a new Data Share Data Lake Gen2 Dataset to be created.
        #[builder(into)]
        pub share_id: pulumi_wasm_rust::Output<String>,
        /// The resource id of the storage account of the data lake file system to be shared with the receiver. Changing this forces a new Data Share Data Lake Gen2 Dataset to be created.
        #[builder(into)]
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DatasetDataLakeGen2Result {
        /// The name of the Data Share Dataset.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The path of the file in the data lake file system to be shared with the receiver. Conflicts with `folder_path` Changing this forces a new Data Share Data Lake Gen2 Dataset to be created.
        pub file_path: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the data lake file system to be shared with the receiver. Changing this forces a new Data Share Data Lake Gen2 Dataset to be created.
        pub file_system_name: pulumi_wasm_rust::Output<String>,
        /// The folder path in the data lake file system to be shared with the receiver. Conflicts with `file_path` Changing this forces a new Data Share Data Lake Gen2 Dataset to be created.
        pub folder_path: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Data Share Data Lake Gen2 Dataset. Changing this forces a new Data Share Data Lake Gen2 Dataset to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the Data Share where this Data Share Data Lake Gen2 Dataset should be created. Changing this forces a new Data Share Data Lake Gen2 Dataset to be created.
        pub share_id: pulumi_wasm_rust::Output<String>,
        /// The resource id of the storage account of the data lake file system to be shared with the receiver. Changing this forces a new Data Share Data Lake Gen2 Dataset to be created.
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DatasetDataLakeGen2Args,
    ) -> DatasetDataLakeGen2Result {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let file_path_binding = args.file_path.get_inner();
        let file_system_name_binding = args.file_system_name.get_inner();
        let folder_path_binding = args.folder_path.get_inner();
        let name_binding = args.name.get_inner();
        let share_id_binding = args.share_id.get_inner();
        let storage_account_id_binding = args.storage_account_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datashare/datasetDataLakeGen2:DatasetDataLakeGen2".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filePath".into(),
                    value: &file_path_binding,
                },
                register_interface::ObjectField {
                    name: "fileSystemName".into(),
                    value: &file_system_name_binding,
                },
                register_interface::ObjectField {
                    name: "folderPath".into(),
                    value: &folder_path_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "shareId".into(),
                    value: &share_id_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "filePath".into(),
                },
                register_interface::ResultField {
                    name: "fileSystemName".into(),
                },
                register_interface::ResultField {
                    name: "folderPath".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "shareId".into(),
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
        DatasetDataLakeGen2Result {
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            file_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filePath").unwrap(),
            ),
            file_system_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileSystemName").unwrap(),
            ),
            folder_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("folderPath").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            share_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shareId").unwrap(),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountId").unwrap(),
            ),
        }
    }
}
