pub mod get_data_catalog_encryption_settings {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDataCatalogEncryptionSettingsArgs {
        /// ID of the Data Catalog. This is typically the AWS account ID.
        #[builder(into)]
        pub catalog_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetDataCatalogEncryptionSettingsResult {
        pub catalog_id: pulumi_wasm_rust::Output<String>,
        /// The security configuration to set. see Data Catalog Encryption Settings.
        pub data_catalog_encryption_settings: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::glue::GetDataCatalogEncryptionSettingsDataCatalogEncryptionSetting,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetDataCatalogEncryptionSettingsArgs,
    ) -> GetDataCatalogEncryptionSettingsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let catalog_id_binding = args.catalog_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:glue/getDataCatalogEncryptionSettings:getDataCatalogEncryptionSettings"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "catalogId".into(),
                    value: &catalog_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "catalogId".into(),
                },
                register_interface::ResultField {
                    name: "dataCatalogEncryptionSettings".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDataCatalogEncryptionSettingsResult {
            catalog_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("catalogId").unwrap(),
            ),
            data_catalog_encryption_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataCatalogEncryptionSettings").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
        }
    }
}