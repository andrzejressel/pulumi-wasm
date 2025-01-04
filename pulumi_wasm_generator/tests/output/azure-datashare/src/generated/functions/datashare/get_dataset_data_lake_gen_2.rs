pub mod get_dataset_data_lake_gen_2 {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDatasetDataLakeGen2Args {
        /// The name of this Data Share Data Lake Gen2 Dataset.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the Data Share where this Data Share Data Lake Gen2 Dataset should be created.
        #[builder(into)]
        pub share_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetDatasetDataLakeGen2Result {
        /// The name of the Data Share Dataset.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The path of the file in the data lake file system to be shared with the receiver.
        pub file_path: pulumi_wasm_rust::Output<String>,
        /// The name of the data lake file system to be shared with the receiver.
        pub file_system_name: pulumi_wasm_rust::Output<String>,
        /// The folder path in the data lake file system to be shared with the receiver.
        pub folder_path: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub share_id: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the storage account of the data lake file system to be shared with the receiver.
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDatasetDataLakeGen2Args) -> GetDatasetDataLakeGen2Result {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let share_id_binding = args.share_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:datashare/getDatasetDataLakeGen2:getDatasetDataLakeGen2"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "shareId".into(),
                    value: &share_id_binding,
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
                    name: "id".into(),
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
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDatasetDataLakeGen2Result {
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
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
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
