#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableExternalDataConfigurationCsvOptions {
    /// Indicates if BigQuery should accept rows
    /// that are missing trailing optional columns.
    #[builder(into, default)]
    #[serde(rename = "allowJaggedRows")]
    pub r#allow_jagged_rows: Box<Option<bool>>,
    /// Indicates if BigQuery should allow
    /// quoted data sections that contain newline characters in a CSV file.
    /// The default value is false.
    #[builder(into, default)]
    #[serde(rename = "allowQuotedNewlines")]
    pub r#allow_quoted_newlines: Box<Option<bool>>,
    /// The character encoding of the data. The supported
    /// values are UTF-8 or ISO-8859-1.
    #[builder(into, default)]
    #[serde(rename = "encoding")]
    pub r#encoding: Box<Option<String>>,
    /// The separator for fields in a CSV file.
    #[builder(into, default)]
    #[serde(rename = "fieldDelimiter")]
    pub r#field_delimiter: Box<Option<String>>,
    /// The value that is used to quote data sections in a
    /// CSV file. If your data does not contain quoted sections, set the
    /// property value to an empty string. If your data contains quoted newline
    /// characters, you must also set the `allow_quoted_newlines` property to true.
    /// The API-side default is `"`, specified in the provider escaped as `\"`. Due to
    /// limitations with default values, this value is required to be
    /// explicitly set.
    #[builder(into)]
    #[serde(rename = "quote")]
    pub r#quote: Box<String>,
    /// The number of rows at the top of a CSV
    /// file that BigQuery will skip when reading the data.
    #[builder(into, default)]
    #[serde(rename = "skipLeadingRows")]
    pub r#skip_leading_rows: Box<Option<i32>>,
}
