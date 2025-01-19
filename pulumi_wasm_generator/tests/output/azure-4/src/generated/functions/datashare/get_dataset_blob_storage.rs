pub mod get_dataset_blob_storage {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDatasetBlobStorageArgs {
        /// The ID of the Data Share in which this Data Share Blob Storage Dataset should be created.
        #[builder(into)]
        pub data_share_id: pulumi_wasm_rust::Output<String>,
        /// The name of this Data Share Blob Storage Dataset.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetDatasetBlobStorageResult {
        /// The name of the storage account container to be shared with the receiver.
        pub container_name: pulumi_wasm_rust::Output<String>,
        pub data_share_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Data Share Dataset.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The path of the file in the storage container to be shared with the receiver.
        pub file_path: pulumi_wasm_rust::Output<String>,
        /// The folder path of the file in the storage container to be shared with the receiver.
        pub folder_path: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The name of the storage account to be shared with the receiver.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `storage_account` block as defined below.
        pub storage_accounts: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::datashare::GetDatasetBlobStorageStorageAccount,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDatasetBlobStorageArgs) -> GetDatasetBlobStorageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_share_id_binding = args.data_share_id.get_inner();
        let name_binding = args.name.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "containerName".into(),
                },
                register_interface::ResultField {
                    name: "dataShareId".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "filePath".into(),
                },
                register_interface::ResultField {
                    name: "folderPath".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "storageAccounts".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDatasetBlobStorageResult {
            container_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerName").unwrap(),
            ),
            data_share_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataShareId").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            file_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filePath").unwrap(),
            ),
            folder_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("folderPath").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            storage_accounts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccounts").unwrap(),
            ),
        }
    }
}
