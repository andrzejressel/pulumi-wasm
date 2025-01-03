#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InsightsReportConfigObjectMetadataReportOptions {
    /// The metadata fields included in an inventory report.
    #[builder(into)]
    #[serde(rename = "metadataFields")]
    pub r#metadata_fields: Box<Vec<String>>,
    /// Options for where the inventory reports are stored.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "storageDestinationOptions")]
    pub r#storage_destination_options: Box<super::super::types::storage::InsightsReportConfigObjectMetadataReportOptionsStorageDestinationOptions>,
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "storageFilters")]
    pub r#storage_filters: Box<Option<super::super::types::storage::InsightsReportConfigObjectMetadataReportOptionsStorageFilters>>,
}
