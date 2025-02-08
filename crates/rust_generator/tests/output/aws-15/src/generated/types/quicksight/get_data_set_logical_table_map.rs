#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDataSetLogicalTableMap {
    #[builder(into)]
    #[serde(rename = "alias")]
    pub r#alias: Box<String>,
    #[builder(into)]
    #[serde(rename = "dataTransforms")]
    pub r#data_transforms: Box<Vec<super::super::types::quicksight::GetDataSetLogicalTableMapDataTransform>>,
    #[builder(into)]
    #[serde(rename = "logicalTableMapId")]
    pub r#logical_table_map_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "sources")]
    pub r#sources: Box<Vec<super::super::types::quicksight::GetDataSetLogicalTableMapSource>>,
}
