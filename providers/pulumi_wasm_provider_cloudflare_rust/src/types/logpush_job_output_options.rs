#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct LogpushJobOutputOptions {
    /// String to be prepended before each batch.
    #[serde(rename = "batchPrefix")]
    pub r#batch_prefix: Box<Option<String>>,
    /// String to be appended after each batch.
    #[serde(rename = "batchSuffix")]
    pub r#batch_suffix: Box<Option<String>>,
    /// Mitigation for CVE-2021-44228. If set to true, will cause all occurrences of ${ in the generated files to be replaced with x{. Defaults to `false`.
    #[serde(rename = "cve20214428")]
    pub r#cve_20214428: Box<Option<bool>>,
    /// String to join fields. This field be ignored when record_template is set. Defaults to `,`.
    #[serde(rename = "fieldDelimiter")]
    pub r#field_delimiter: Box<Option<String>>,
    /// List of field names to be included in the Logpush output.
    #[serde(rename = "fieldNames")]
    pub r#field_names: Box<Option<Vec<String>>>,
    /// Specifies the output type. Available values: `ndjson`, `csv`. Defaults to `ndjson`.
    #[serde(rename = "outputType")]
    pub r#output_type: Box<Option<String>>,
    /// String to be inserted in-between the records as separator.
    #[serde(rename = "recordDelimiter")]
    pub r#record_delimiter: Box<Option<String>>,
    /// String to be prepended before each record. Defaults to `{`.
    #[serde(rename = "recordPrefix")]
    pub r#record_prefix: Box<Option<String>>,
    /// String to be appended after each record. Defaults to `}`.
    #[serde(rename = "recordSuffix")]
    pub r#record_suffix: Box<Option<String>>,
    /// String to use as template for each record instead of the default comma-separated list.
    #[serde(rename = "recordTemplate")]
    pub r#record_template: Box<Option<String>>,
    /// Specifies the sampling rate. Defaults to `1`.
    #[serde(rename = "sampleRate")]
    pub r#sample_rate: Box<Option<f64>>,
    /// Specifies the format for timestamps. Available values: `unixnano`, `unix`, `rfc3339`. Defaults to `unixnano`.
    #[serde(rename = "timestampFormat")]
    pub r#timestamp_format: Box<Option<String>>,
}
