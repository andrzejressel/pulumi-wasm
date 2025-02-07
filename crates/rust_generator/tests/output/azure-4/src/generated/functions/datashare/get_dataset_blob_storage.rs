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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDatasetBlobStorageArgs,
    ) -> GetDatasetBlobStorageResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let data_share_id_binding = args.data_share_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:datashare/getDatasetBlobStorage:getDatasetBlobStorage".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataShareId".into(),
                    value: &data_share_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDatasetBlobStorageResult {
            container_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("containerName"),
            ),
            data_share_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataShareId"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            file_path: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filePath"),
            ),
            folder_path: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("folderPath"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            storage_accounts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccounts"),
            ),
        }
    }
}
