/// Manages a Data Share Blob Storage Dataset.
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
///       accountTier: Standard
///       accountReplicationType: RAGRS
///   exampleContainer:
///     type: azure:storage:Container
///     name: example
///     properties:
///       name: example-sc
///       storageAccountName: ${exampleAccount2.name}
///       containerAccessType: container
///   exampleAssignment:
///     type: azure:authorization:Assignment
///     name: example
///     properties:
///       scope: ${exampleAccount2.id}
///       roleDefinitionName: Storage Blob Data Reader
///       principalId: ${example.objectId}
///   exampleDatasetBlobStorage:
///     type: azure:datashare:DatasetBlobStorage
///     name: example
///     properties:
///       name: example-dsbsds-file
///       dataShareId: ${exampleShare.id}
///       containerName: ${exampleContainer.name}
///       storageAccount:
///         name: ${exampleAccount2.name}
///         resourceGroupName: ${exampleAccount2.resourceGroupName}
///         subscriptionId: 00000000-0000-0000-0000-000000000000
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
/// Data Share Blob Storage Datasets can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datashare/datasetBlobStorage:DatasetBlobStorage example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DataShare/accounts/account1/shares/share1/dataSets/dataSet1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dataset_blob_storage {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatasetBlobStorageArgs {
        /// The name of the storage account container to be shared with the receiver. Changing this forces a new Data Share Blob Storage Dataset to be created.
        #[builder(into)]
        pub container_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Data Share in which this Data Share Blob Storage Dataset should be created. Changing this forces a new Data Share Blob Storage Dataset to be created.
        #[builder(into)]
        pub data_share_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The path of the file in the storage container to be shared with the receiver. Changing this forces a new Data Share Blob Storage Dataset to be created.
        #[builder(into, default)]
        pub file_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The path of the folder in the storage container to be shared with the receiver. Changing this forces a new Data Share Blob Storage Dataset to be created.
        #[builder(into, default)]
        pub folder_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Data Share Blob Storage Dataset. Changing this forces a new Data Share Blob Storage Dataset to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `storage_account` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_account: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::datashare::DatasetBlobStorageStorageAccount,
        >,
    }
    #[allow(dead_code)]
    pub struct DatasetBlobStorageResult {
        /// The name of the storage account container to be shared with the receiver. Changing this forces a new Data Share Blob Storage Dataset to be created.
        pub container_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Data Share in which this Data Share Blob Storage Dataset should be created. Changing this forces a new Data Share Blob Storage Dataset to be created.
        pub data_share_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Data Share Dataset.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The path of the file in the storage container to be shared with the receiver. Changing this forces a new Data Share Blob Storage Dataset to be created.
        pub file_path: pulumi_gestalt_rust::Output<Option<String>>,
        /// The path of the folder in the storage container to be shared with the receiver. Changing this forces a new Data Share Blob Storage Dataset to be created.
        pub folder_path: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Data Share Blob Storage Dataset. Changing this forces a new Data Share Blob Storage Dataset to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `storage_account` block as defined below. Changing this forces a new resource to be created.
        pub storage_account: pulumi_gestalt_rust::Output<
            super::super::types::datashare::DatasetBlobStorageStorageAccount,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DatasetBlobStorageArgs,
    ) -> DatasetBlobStorageResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let container_name_binding = args.container_name.get_output(context);
        let data_share_id_binding = args.data_share_id.get_output(context);
        let file_path_binding = args.file_path.get_output(context);
        let folder_path_binding = args.folder_path.get_output(context);
        let name_binding = args.name.get_output(context);
        let storage_account_binding = args.storage_account.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:datashare/datasetBlobStorage:DatasetBlobStorage".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerName".into(),
                    value: container_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataShareId".into(),
                    value: data_share_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filePath".into(),
                    value: file_path_binding.get_id(),
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
                    name: "storageAccount".into(),
                    value: storage_account_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DatasetBlobStorageResult {
            container_name: o.get_field("containerName"),
            data_share_id: o.get_field("dataShareId"),
            display_name: o.get_field("displayName"),
            file_path: o.get_field("filePath"),
            folder_path: o.get_field("folderPath"),
            name: o.get_field("name"),
            storage_account: o.get_field("storageAccount"),
        }
    }
}
