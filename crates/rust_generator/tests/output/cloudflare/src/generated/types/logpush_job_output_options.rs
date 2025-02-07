#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LogpushJobOutputOptions {
    /// String to be prepended before each batch.
    #[builder(into, default)]
    #[serde(rename = "batchPrefix")]
    pub r#batch_prefix: Box<Option<String>>,
    /// String to be appended after each batch.
    #[builder(into, default)]
    #[serde(rename = "batchSuffix")]
    pub r#batch_suffix: Box<Option<String>>,
    /// Mitigation for CVE-2021-44228. If set to true, will cause all occurrences of ${ in the generated files to be replaced with x{. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "cve20214428")]
    pub r#cve_20214428: Box<Option<bool>>,
    /// String to join fields. This field be ignored when record_template is set. Defaults to `,`.
    #[builder(into, default)]
    #[serde(rename = "fieldDelimiter")]
    pub r#field_delimiter: Box<Option<String>>,
    /// List of field names to be included in the Logpush output.
    #[builder(into, default)]
    #[serde(rename = "fieldNames")]
    pub r#field_names: Box<Option<Vec<String>>>,
    /// Specifies the output type. Available values: `ndjson`, `csv`. Defaults to `ndjson`.
    #[builder(into, default)]
    #[serde(rename = "outputType")]
    pub r#output_type: Box<Option<String>>,
    /// String to be inserted in-between the records as separator.
    #[builder(into, default)]
    #[serde(rename = "recordDelimiter")]
    pub r#record_delimiter: Box<Option<String>>,
    /// String to be prepended before each record. Defaults to `{`.
    #[builder(into, default)]
    #[serde(rename = "recordPrefix")]
    pub r#record_prefix: Box<Option<String>>,
    /// String to be appended after each record. Defaults to `}
    /// `.
    #[builder(into, default)]
    #[serde(rename = "recordSuffix")]
    pub r#record_suffix: Box<Option<String>>,
    /// String to use as template for each record instead of the default comma-separated list.
    #[builder(into, default)]
    #[serde(rename = "recordTemplate")]
    pub r#record_template: Box<Option<String>>,
    /// Specifies the sampling rate. Defaults to `1`.
    #[builder(into, default)]
    #[serde(rename = "sampleRate")]
    pub r#sample_rate: Box<Option<f64>>,
    /// Specifies the format for timestamps. Available values: `unixnano`, `unix`, `rfc3339`. Defaults to `unixnano`.
    #[builder(into, default)]
    #[serde(rename = "timestampFormat")]
    pub r#timestamp_format: Box<Option<String>>,
}
