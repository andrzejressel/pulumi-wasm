#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSetLogicalTableMapDataTransform {
    /// A transform operation that casts a column to a different type. See cast_column_type_operation.
    #[builder(into, default)]
    #[serde(rename = "castColumnTypeOperation")]
    pub r#cast_column_type_operation: Box<Option<super::super::types::quicksight::DataSetLogicalTableMapDataTransformCastColumnTypeOperation>>,
    /// An operation that creates calculated columns. Columns created in one such operation form a lexical closure. See create_columns_operation.
    #[builder(into, default)]
    #[serde(rename = "createColumnsOperation")]
    pub r#create_columns_operation: Box<Option<super::super::types::quicksight::DataSetLogicalTableMapDataTransformCreateColumnsOperation>>,
    /// An operation that filters rows based on some condition. See filter_operation.
    #[builder(into, default)]
    #[serde(rename = "filterOperation")]
    pub r#filter_operation: Box<Option<super::super::types::quicksight::DataSetLogicalTableMapDataTransformFilterOperation>>,
    /// An operation that projects columns. Operations that come after a projection can only refer to projected columns. See project_operation.
    #[builder(into, default)]
    #[serde(rename = "projectOperation")]
    pub r#project_operation: Box<Option<super::super::types::quicksight::DataSetLogicalTableMapDataTransformProjectOperation>>,
    /// An operation that renames a column. See rename_column_operation.
    #[builder(into, default)]
    #[serde(rename = "renameColumnOperation")]
    pub r#rename_column_operation: Box<Option<super::super::types::quicksight::DataSetLogicalTableMapDataTransformRenameColumnOperation>>,
    /// An operation that tags a column with additional information. See tag_column_operation.
    #[builder(into, default)]
    #[serde(rename = "tagColumnOperation")]
    pub r#tag_column_operation: Box<Option<super::super::types::quicksight::DataSetLogicalTableMapDataTransformTagColumnOperation>>,
    /// A transform operation that removes tags associated with a column. See untag_column_operation.
    #[builder(into, default)]
    #[serde(rename = "untagColumnOperation")]
    pub r#untag_column_operation: Box<Option<super::super::types::quicksight::DataSetLogicalTableMapDataTransformUntagColumnOperation>>,
}
