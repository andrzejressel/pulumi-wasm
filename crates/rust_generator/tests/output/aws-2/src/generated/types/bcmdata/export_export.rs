#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ExportExport {
    /// Data query for this specific data export. See the `data_query` argument reference below.
    #[builder(into, default)]
    #[serde(rename = "dataQueries")]
    pub r#data_queries: Box<Option<Vec<super::super::types::bcmdata::ExportExportDataQuery>>>,
    /// Description for this specific data export.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Destination configuration for this specific data export. See the `destination_configurations` argument reference below.
    #[builder(into, default)]
    #[serde(rename = "destinationConfigurations")]
    pub r#destination_configurations: Box<Option<Vec<super::super::types::bcmdata::ExportExportDestinationConfiguration>>>,
    /// Amazon Resource Name (ARN) for this export.
    #[builder(into, default)]
    #[serde(rename = "exportArn")]
    pub r#export_arn: Box<Option<String>>,
    /// Name of this specific data export.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Cadence for Amazon Web Services to update the export in your S3 bucket. See the `refresh_cadence` argument reference below.
    #[builder(into, default)]
    #[serde(rename = "refreshCadences")]
    pub r#refresh_cadences: Box<Option<Vec<super::super::types::bcmdata::ExportExportRefreshCadence>>>,
}
