/// Provides a Glue Data Catalog Encryption Settings resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_catalog_encryption_settings::create(
///         "example",
///         DataCatalogEncryptionSettingsArgs::builder()
///             .data_catalog_encryption_settings(
///                 DataCatalogEncryptionSettingsDataCatalogEncryptionSettings::builder()
///                     .connectionPasswordEncryption(
///                         DataCatalogEncryptionSettingsDataCatalogEncryptionSettingsConnectionPasswordEncryption::builder()
///                             .awsKmsKeyId("${test.arn}")
///                             .returnConnectionPasswordEncrypted(true)
///                             .build_struct(),
///                     )
///                     .encryptionAtRest(
///                         DataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEncryptionAtRest::builder()
///                             .catalogEncryptionMode("SSE-KMS")
///                             .catalogEncryptionServiceRole("${role.test.arn}")
///                             .sseAwsKmsKeyId("${test.arn}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glue Data Catalog Encryption Settings using `CATALOG-ID` (AWS account ID if not custom). For example:
///
/// ```sh
/// $ pulumi import aws:glue/dataCatalogEncryptionSettings:DataCatalogEncryptionSettings example 123456789012
/// ```
pub mod data_catalog_encryption_settings {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataCatalogEncryptionSettingsArgs {
        /// The ID of the Data Catalog to set the security configuration for. If none is provided, the AWS account ID is used by default.
        #[builder(into, default)]
        pub catalog_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The security configuration to set. see Data Catalog Encryption Settings.
        #[builder(into)]
        pub data_catalog_encryption_settings: pulumi_wasm_rust::Output<
            super::super::types::glue::DataCatalogEncryptionSettingsDataCatalogEncryptionSettings,
        >,
    }
    #[allow(dead_code)]
    pub struct DataCatalogEncryptionSettingsResult {
        /// The ID of the Data Catalog to set the security configuration for. If none is provided, the AWS account ID is used by default.
        pub catalog_id: pulumi_wasm_rust::Output<String>,
        /// The security configuration to set. see Data Catalog Encryption Settings.
        pub data_catalog_encryption_settings: pulumi_wasm_rust::Output<
            super::super::types::glue::DataCatalogEncryptionSettingsDataCatalogEncryptionSettings,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DataCatalogEncryptionSettingsArgs,
    ) -> DataCatalogEncryptionSettingsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let catalog_id_binding = args.catalog_id.get_inner();
        let data_catalog_encryption_settings_binding = args
            .data_catalog_encryption_settings
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:glue/dataCatalogEncryptionSettings:DataCatalogEncryptionSettings"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "catalogId".into(),
                    value: &catalog_id_binding,
                },
                register_interface::ObjectField {
                    name: "dataCatalogEncryptionSettings".into(),
                    value: &data_catalog_encryption_settings_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "catalogId".into(),
                },
                register_interface::ResultField {
                    name: "dataCatalogEncryptionSettings".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DataCatalogEncryptionSettingsResult {
            catalog_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("catalogId").unwrap(),
            ),
            data_catalog_encryption_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataCatalogEncryptionSettings").unwrap(),
            ),
        }
    }
}
