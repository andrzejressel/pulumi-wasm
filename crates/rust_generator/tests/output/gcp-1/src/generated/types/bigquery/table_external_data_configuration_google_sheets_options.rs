#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TableExternalDataConfigurationGoogleSheetsOptions {
    /// Range of a sheet to query from. Only used when
    /// non-empty. At least one of `range` or `skip_leading_rows` must be set.
    /// Typical format: "sheet_name!top_left_cell_id:bottom_right_cell_id"
    /// For example: "sheet1!A1:B20"
    #[builder(into, default)]
    #[serde(rename = "range")]
    pub r#range: Box<Option<String>>,
    /// The number of rows at the top of the sheet
    /// that BigQuery will skip when reading the data. At least one of `range` or
    /// `skip_leading_rows` must be set.
    #[builder(into, default)]
    #[serde(rename = "skipLeadingRows")]
    pub r#skip_leading_rows: Box<Option<i32>>,
}
