#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StaticWebLayerCloudwatchConfigurationLogStream {
    #[builder(into, default)]
    #[serde(rename = "batchCount")]
    pub r#batch_count: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "batchSize")]
    pub r#batch_size: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "bufferDuration")]
    pub r#buffer_duration: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "datetimeFormat")]
    pub r#datetime_format: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "encoding")]
    pub r#encoding: Box<Option<String>>,
    #[builder(into)]
    #[serde(rename = "file")]
    pub r#file: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "fileFingerprintLines")]
    pub r#file_fingerprint_lines: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "initialPosition")]
    pub r#initial_position: Box<Option<String>>,
    #[builder(into)]
    #[serde(rename = "logGroupName")]
    pub r#log_group_name: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "multilineStartPattern")]
    pub r#multiline_start_pattern: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Box<Option<String>>,
}
