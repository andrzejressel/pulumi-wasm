#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataSetLogicalTableMapDataTransformTagColumnOperationTag {
    #[builder(into)]
    #[serde(rename = "columnDescriptions")]
    pub r#column_descriptions: Box<Vec<super::super::types::quicksight::GetDataSetLogicalTableMapDataTransformTagColumnOperationTagColumnDescription>>,
    #[builder(into)]
    #[serde(rename = "columnGeographicRole")]
    pub r#column_geographic_role: Box<String>,
}
