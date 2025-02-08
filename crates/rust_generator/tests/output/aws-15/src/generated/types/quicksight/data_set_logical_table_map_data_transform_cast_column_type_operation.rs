#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSetLogicalTableMapDataTransformCastColumnTypeOperation {
    /// Column name.
    #[builder(into)]
    #[serde(rename = "columnName")]
    pub r#column_name: Box<String>,
    /// When casting a column from string to datetime type, you can supply a string in a format supported by Amazon QuickSight to denote the source data format.
    #[builder(into, default)]
    #[serde(rename = "format")]
    pub r#format: Box<Option<String>>,
    /// New column data type. Valid values are `STRING`, `INTEGER`, `DECIMAL`, `DATETIME`.
    #[builder(into)]
    #[serde(rename = "newColumnType")]
    pub r#new_column_type: Box<String>,
}
