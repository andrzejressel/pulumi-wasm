#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetDataSetLogicalTableMapDataTransformTagColumnOperation {
    #[builder(into)]
    #[serde(rename = "columnName")]
    pub r#column_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Vec<super::super::types::quicksight::GetDataSetLogicalTableMapDataTransformTagColumnOperationTag>>,
}
