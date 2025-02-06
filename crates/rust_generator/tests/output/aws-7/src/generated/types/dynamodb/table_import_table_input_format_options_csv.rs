#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TableImportTableInputFormatOptionsCsv {
    /// The delimiter used for separating items in the CSV file being imported.
    #[builder(into, default)]
    #[serde(rename = "delimiter")]
    pub r#delimiter: Box<Option<String>>,
    /// List of the headers used to specify a common header for all source CSV files being imported.
    #[builder(into, default)]
    #[serde(rename = "headerLists")]
    pub r#header_lists: Box<Option<Vec<String>>>,
}
