#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StreamDestinationConfigBigqueryDestinationConfigSourceHierarchyDatasets {
    /// Dataset template used for dynamic dataset creation.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "datasetTemplate")]
    pub r#dataset_template: Box<super::super::types::datastream::StreamDestinationConfigBigqueryDestinationConfigSourceHierarchyDatasetsDatasetTemplate>,
}
