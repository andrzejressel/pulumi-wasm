#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MetastoreServiceMetadataIntegration {
    /// The integration config for the Data Catalog service.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dataCatalogConfig")]
    pub r#data_catalog_config: Box<super::super::types::dataproc::MetastoreServiceMetadataIntegrationDataCatalogConfig>,
}
