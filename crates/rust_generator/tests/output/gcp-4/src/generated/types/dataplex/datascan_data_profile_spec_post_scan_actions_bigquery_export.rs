#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DatascanDataProfileSpecPostScanActionsBigqueryExport {
    /// The BigQuery table to export DataProfileScan results to.
    /// Format://bigquery.googleapis.com/projects/PROJECT_ID/datasets/DATASET_ID/tables/TABLE_ID
    #[builder(into, default)]
    #[serde(rename = "resultsTable")]
    pub r#results_table: Box<Option<String>>,
}
