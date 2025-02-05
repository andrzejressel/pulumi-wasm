#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataSetLogicalTableMapDataTransform {
    #[builder(into)]
    #[serde(rename = "castColumnTypeOperations")]
    pub r#cast_column_type_operations: Box<Vec<super::super::types::quicksight::GetDataSetLogicalTableMapDataTransformCastColumnTypeOperation>>,
    #[builder(into)]
    #[serde(rename = "createColumnsOperations")]
    pub r#create_columns_operations: Box<Vec<super::super::types::quicksight::GetDataSetLogicalTableMapDataTransformCreateColumnsOperation>>,
    #[builder(into)]
    #[serde(rename = "filterOperations")]
    pub r#filter_operations: Box<Vec<super::super::types::quicksight::GetDataSetLogicalTableMapDataTransformFilterOperation>>,
    #[builder(into)]
    #[serde(rename = "projectOperations")]
    pub r#project_operations: Box<Vec<super::super::types::quicksight::GetDataSetLogicalTableMapDataTransformProjectOperation>>,
    #[builder(into)]
    #[serde(rename = "renameColumnOperations")]
    pub r#rename_column_operations: Box<Vec<super::super::types::quicksight::GetDataSetLogicalTableMapDataTransformRenameColumnOperation>>,
    #[builder(into)]
    #[serde(rename = "tagColumnOperations")]
    pub r#tag_column_operations: Box<Vec<super::super::types::quicksight::GetDataSetLogicalTableMapDataTransformTagColumnOperation>>,
    #[builder(into)]
    #[serde(rename = "untagColumnOperations")]
    pub r#untag_column_operations: Box<Vec<super::super::types::quicksight::GetDataSetLogicalTableMapDataTransformUntagColumnOperation>>,
}
