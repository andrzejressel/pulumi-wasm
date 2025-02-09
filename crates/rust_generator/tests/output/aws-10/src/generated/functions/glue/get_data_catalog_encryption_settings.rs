#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_data_catalog_encryption_settings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDataCatalogEncryptionSettingsArgs {
        /// ID of the Data Catalog. This is typically the AWS account ID.
        #[builder(into)]
        pub catalog_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDataCatalogEncryptionSettingsResult {
        pub catalog_id: pulumi_gestalt_rust::Output<String>,
        /// The security configuration to set. see Data Catalog Encryption Settings.
        pub data_catalog_encryption_settings: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::glue::GetDataCatalogEncryptionSettingsDataCatalogEncryptionSetting,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDataCatalogEncryptionSettingsArgs,
    ) -> GetDataCatalogEncryptionSettingsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let catalog_id_binding = args.catalog_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:glue/getDataCatalogEncryptionSettings:getDataCatalogEncryptionSettings"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "catalogId".into(),
                    value: catalog_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDataCatalogEncryptionSettingsResult {
            catalog_id: o.get_field("catalogId"),
            data_catalog_encryption_settings: o
                .get_field("dataCatalogEncryptionSettings"),
            id: o.get_field("id"),
        }
    }
}
