pub mod get_dataset_data_lake_gen_2 {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDatasetDataLakeGen2Args {
        /// The name of this Data Share Data Lake Gen2 Dataset.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The resource ID of the Data Share where this Data Share Data Lake Gen2 Dataset should be created.
        #[builder(into)]
        pub share_id: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetDatasetDataLakeGen2Args,
    ) -> GetDatasetDataLakeGen2Result {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let share_id_binding = args.share_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:datashare/getDatasetDataLakeGen2:getDatasetDataLakeGen2"
                .into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDatasetDataLakeGen2Result {
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            file_path: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("filePath"),
            ),
            file_system_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fileSystemName"),
            ),
            folder_path: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("folderPath"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            share_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("shareId"),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageAccountId"),
            ),
        }
    }
}
