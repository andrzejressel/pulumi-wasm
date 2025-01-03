#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DatasetAccessAuthorizedDatasetDataset {
    /// The ID of the dataset containing this table.
    #[builder(into)]
    #[serde(rename = "datasetId")]
    pub r#dataset_id: Box<String>,
    /// The ID of the project containing this table.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: Box<String>,
}
