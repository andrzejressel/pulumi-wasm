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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dataset_data_lake_gen_2 {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatasetDataLakeGen2Args {
        /// The path of the file in the data lake file system to be shared with the receiver. Conflicts with `folder_path` Changing this forces a new Data Share Data Lake Gen2 Dataset to be created.
        #[builder(into, default)]
        pub file_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the data lake file system to be shared with the receiver. Changing this forces a new Data Share Data Lake Gen2 Dataset to be created.
        #[builder(into)]
        pub file_system_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The folder path in the data lake file system to be shared with the receiver. Conflicts with `file_path` Changing this forces a new Data Share Data Lake Gen2 Dataset to be created.
        #[builder(into, default)]
        pub folder_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Data Share Data Lake Gen2 Dataset. Changing this forces a new Data Share Data Lake Gen2 Dataset to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource ID of the Data Share where this Data Share Data Lake Gen2 Dataset should be created. Changing this forces a new Data Share Data Lake Gen2 Dataset to be created.
        #[builder(into)]
        pub share_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource id of the storage account of the data lake file system to be shared with the receiver. Changing this forces a new Data Share Data Lake Gen2 Dataset to be created.
        #[builder(into)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DatasetDataLakeGen2Result {
        /// The name of the Data Share Dataset.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The path of the file in the data lake file system to be shared with the receiver. Conflicts with `folder_path` Changing this forces a new Data Share Data Lake Gen2 Dataset to be created.
        pub file_path: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the data lake file system to be shared with the receiver. Changing this forces a new Data Share Data Lake Gen2 Dataset to be created.
        pub file_system_name: pulumi_gestalt_rust::Output<String>,
        /// The folder path in the data lake file system to be shared with the receiver. Conflicts with `file_path` Changing this forces a new Data Share Data Lake Gen2 Dataset to be created.
        pub folder_path: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Data Share Data Lake Gen2 Dataset. Changing this forces a new Data Share Data Lake Gen2 Dataset to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The resource ID of the Data Share where this Data Share Data Lake Gen2 Dataset should be created. Changing this forces a new Data Share Data Lake Gen2 Dataset to be created.
        pub share_id: pulumi_gestalt_rust::Output<String>,
        /// The resource id of the storage account of the data lake file system to be shared with the receiver. Changing this forces a new Data Share Data Lake Gen2 Dataset to be created.
        pub storage_account_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DatasetDataLakeGen2Args,
    ) -> DatasetDataLakeGen2Result {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let file_path_binding = args.file_path.get_output(context);
        let file_system_name_binding = args.file_system_name.get_output(context);
        let folder_path_binding = args.folder_path.get_output(context);
        let name_binding = args.name.get_output(context);
        let share_id_binding = args.share_id.get_output(context);
        let storage_account_id_binding = args.storage_account_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:datashare/datasetDataLakeGen2:DatasetDataLakeGen2".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filePath".into(),
                    value: file_path_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fileSystemName".into(),
                    value: file_system_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "folderPath".into(),
                    value: folder_path_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shareId".into(),
                    value: share_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountId".into(),
                    value: storage_account_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DatasetDataLakeGen2Result {
            display_name: o.get_field("displayName"),
            file_path: o.get_field("filePath"),
            file_system_name: o.get_field("fileSystemName"),
            folder_path: o.get_field("folderPath"),
            name: o.get_field("name"),
            share_id: o.get_field("shareId"),
            storage_account_id: o.get_field("storageAccountId"),
        }
    }
}
