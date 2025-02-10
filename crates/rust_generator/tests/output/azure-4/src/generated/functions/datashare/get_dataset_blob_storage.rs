#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_dataset_blob_storage {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDatasetBlobStorageArgs {
        /// The ID of the Data Share in which this Data Share Blob Storage Dataset should be created.
        #[builder(into)]
        pub data_share_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of this Data Share Blob Storage Dataset.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDatasetBlobStorageResult {
        /// The name of the storage account container to be shared with the receiver.
        pub container_name: pulumi_gestalt_rust::Output<String>,
        pub data_share_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Data Share Dataset.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The path of the file in the storage container to be shared with the receiver.
        pub file_path: pulumi_gestalt_rust::Output<String>,
        /// The folder path of the file in the storage container to be shared with the receiver.
        pub folder_path: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The name of the storage account to be shared with the receiver.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `storage_account` block as defined below.
        pub storage_accounts: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::datashare::GetDatasetBlobStorageStorageAccount,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDatasetBlobStorageArgs,
    ) -> GetDatasetBlobStorageResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let data_share_id_binding = args.data_share_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:datashare/getDatasetBlobStorage:getDatasetBlobStorage".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataShareId".into(),
                    value: data_share_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDatasetBlobStorageResult {
            container_name: o.get_field("containerName"),
            data_share_id: o.get_field("dataShareId"),
            display_name: o.get_field("displayName"),
            file_path: o.get_field("filePath"),
            folder_path: o.get_field("folderPath"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            storage_accounts: o.get_field("storageAccounts"),
        }
    }
}
