#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataCellsFilterTableDataRowFilter {
    /// (Optional) A wildcard that matches all rows.
    #[builder(into, default)]
    #[serde(rename = "allRowsWildcard")]
    pub r#all_rows_wildcard: Box<Option<super::super::types::lakeformation::DataCellsFilterTableDataRowFilterAllRowsWildcard>>,
    /// (Optional) A filter expression.
    #[builder(into, default)]
    #[serde(rename = "filterExpression")]
    pub r#filter_expression: Box<Option<String>>,
}