#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct InsightsReportConfigCsvOptions {
    /// The delimiter used to separate the fields in the inventory report CSV file.
    #[builder(into, default)]
    #[serde(rename = "delimiter")]
    pub r#delimiter: Box<Option<String>>,
    /// The boolean that indicates whether or not headers are included in the inventory report CSV file.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "headerRequired")]
    pub r#header_required: Box<Option<bool>>,
    /// The character used to separate the records in the inventory report CSV file.
    #[builder(into, default)]
    #[serde(rename = "recordSeparator")]
    pub r#record_separator: Box<Option<String>>,
}
