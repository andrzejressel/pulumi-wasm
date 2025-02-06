#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ListingBigqueryDatasetSelectedResource {
    /// Format: For table: projects/{projectId}/datasets/{datasetId}/tables/{tableId} Example:"projects/test_project/datasets/test_dataset/tables/test_table"
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "table")]
    pub r#table: Box<Option<String>>,
}
