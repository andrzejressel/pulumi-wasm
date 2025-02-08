#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ZoneDiscoverySpecCsvOptions {
    /// Optional. The delimiter being used to separate values. This defaults to ','.
    #[builder(into, default)]
    #[serde(rename = "delimiter")]
    pub r#delimiter: Box<Option<String>>,
    /// Optional. Whether to disable the inference of data type for CSV data. If true, all columns will be registered as strings.
    #[builder(into, default)]
    #[serde(rename = "disableTypeInference")]
    pub r#disable_type_inference: Box<Option<bool>>,
    /// Optional. The character encoding of the data. The default is UTF-8.
    #[builder(into, default)]
    #[serde(rename = "encoding")]
    pub r#encoding: Box<Option<String>>,
    /// Optional. The number of rows to interpret as header rows that should be skipped when reading data rows.
    #[builder(into, default)]
    #[serde(rename = "headerRows")]
    pub r#header_rows: Box<Option<i32>>,
}
