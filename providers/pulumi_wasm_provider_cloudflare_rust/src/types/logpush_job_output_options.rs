#[derive(serde::Serialize)]
pub struct LogpushJobOutputOptions {
    #[serde(rename = "batchPrefix")]
    pub r#batch_prefix: Box<Option<String>>,
    #[serde(rename = "batchSuffix")]
    pub r#batch_suffix: Box<Option<String>>,
    #[serde(rename = "cve20214428")]
    pub r#cve_20214428: Box<Option<bool>>,
    #[serde(rename = "fieldDelimiter")]
    pub r#field_delimiter: Box<Option<String>>,
    #[serde(rename = "fieldNames")]
    pub r#field_names: Box<Option<Vec<String>>>,
    #[serde(rename = "outputType")]
    pub r#output_type: Box<Option<String>>,
    #[serde(rename = "recordDelimiter")]
    pub r#record_delimiter: Box<Option<String>>,
    #[serde(rename = "recordPrefix")]
    pub r#record_prefix: Box<Option<String>>,
    #[serde(rename = "recordSuffix")]
    pub r#record_suffix: Box<Option<String>>,
    #[serde(rename = "recordTemplate")]
    pub r#record_template: Box<Option<String>>,
    #[serde(rename = "sampleRate")]
    pub r#sample_rate: Box<Option<f64>>,
    #[serde(rename = "timestampFormat")]
    pub r#timestamp_format: Box<Option<String>>,
}
