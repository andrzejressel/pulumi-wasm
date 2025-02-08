#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DataSetLogicalTableMapDataTransformCreateColumnsOperationColumn {
    /// A unique ID to identify a calculated column. During a dataset update, if the column ID of a calculated column matches that of an existing calculated column, Amazon QuickSight preserves the existing calculated column.
    #[builder(into)]
    #[serde(rename = "columnId")]
    pub r#column_id: Box<String>,
    /// Column name.
    #[builder(into)]
    #[serde(rename = "columnName")]
    pub r#column_name: Box<String>,
    /// An expression that defines the calculated column.
    #[builder(into)]
    #[serde(rename = "expression")]
    pub r#expression: Box<String>,
}
