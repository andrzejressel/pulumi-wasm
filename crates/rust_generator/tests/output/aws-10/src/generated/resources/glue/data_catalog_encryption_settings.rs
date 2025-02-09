/// Provides a Glue Data Catalog Encryption Settings resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod data_catalog_encryption_settings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataCatalogEncryptionSettingsArgs {
        /// The ID of the Data Catalog to set the security configuration for. If none is provided, the AWS account ID is used by default.
        #[builder(into, default)]
        pub catalog_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The security configuration to set. see Data Catalog Encryption Settings.
        #[builder(into)]
        pub data_catalog_encryption_settings: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::glue::DataCatalogEncryptionSettingsDataCatalogEncryptionSettings,
        >,
    }
    #[allow(dead_code)]
    pub struct DataCatalogEncryptionSettingsResult {
        /// The ID of the Data Catalog to set the security configuration for. If none is provided, the AWS account ID is used by default.
        pub catalog_id: pulumi_gestalt_rust::Output<String>,
        /// The security configuration to set. see Data Catalog Encryption Settings.
        pub data_catalog_encryption_settings: pulumi_gestalt_rust::Output<
            super::super::types::glue::DataCatalogEncryptionSettingsDataCatalogEncryptionSettings,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataCatalogEncryptionSettingsArgs,
    ) -> DataCatalogEncryptionSettingsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let catalog_id_binding = args.catalog_id.get_output(context);
        let data_catalog_encryption_settings_binding = args
            .data_catalog_encryption_settings
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:glue/dataCatalogEncryptionSettings:DataCatalogEncryptionSettings"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "catalogId".into(),
                    value: catalog_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataCatalogEncryptionSettings".into(),
                    value: data_catalog_encryption_settings_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DataCatalogEncryptionSettingsResult {
            catalog_id: o.get_field("catalogId"),
            data_catalog_encryption_settings: o
                .get_field("dataCatalogEncryptionSettings"),
        }
    }
}
