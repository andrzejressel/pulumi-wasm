#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LinkedDatasetBigqueryDataset {
    /// (Output)
    /// Output only. The full resource name of the BigQuery dataset. The DATASET_ID will match the ID
    /// of the link, so the link must match the naming restrictions of BigQuery datasets
    /// (alphanumeric characters and underscores only). The dataset will have a resource path of
    /// "bigquery.googleapis.com/projects/[PROJECT_ID]/datasets/[DATASET_ID]"
    #[builder(into, default)]
    #[serde(rename = "datasetId")]
    pub r#dataset_id: Box<Option<String>>,
}
