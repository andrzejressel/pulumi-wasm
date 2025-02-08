#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CustomLayerCloudwatchConfigurationLogStream {
    /// Specifies the max number of log events in a batch, up to `10000`. The default value is `1000`.
    #[builder(into, default)]
    #[serde(rename = "batchCount")]
    pub r#batch_count: Box<Option<i32>>,
    /// Specifies the maximum size of log events in a batch, in bytes, up to `1048576` bytes. The default value is `32768` bytes.
    #[builder(into, default)]
    #[serde(rename = "batchSize")]
    pub r#batch_size: Box<Option<i32>>,
    /// Specifies the time duration for the batching of log events. The minimum value is `5000` and default value is `5000`.
    #[builder(into, default)]
    #[serde(rename = "bufferDuration")]
    pub r#buffer_duration: Box<Option<i32>>,
    /// Specifies how the timestamp is extracted from logs. For more information, see the CloudWatch Logs Agent Reference (https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/AgentReference.html).
    #[builder(into, default)]
    #[serde(rename = "datetimeFormat")]
    pub r#datetime_format: Box<Option<String>>,
    /// Specifies the encoding of the log file so that the file can be read correctly. The default is `utf_8`.
    #[builder(into, default)]
    #[serde(rename = "encoding")]
    pub r#encoding: Box<Option<String>>,
    /// Specifies log files that you want to push to CloudWatch Logs. File can point to a specific file or multiple files (by using wild card characters such as /var/log/system.log*).
    #[builder(into)]
    #[serde(rename = "file")]
    pub r#file: Box<String>,
    /// Specifies the range of lines for identifying a file. The valid values are one number, or two dash-delimited numbers, such as `1`, `2-5`. The default value is `1`.
    #[builder(into, default)]
    #[serde(rename = "fileFingerprintLines")]
    pub r#file_fingerprint_lines: Box<Option<String>>,
    /// Specifies where to start to read data (`start_of_file` or `end_of_file`). The default is `start_of_file`.
    #[builder(into, default)]
    #[serde(rename = "initialPosition")]
    pub r#initial_position: Box<Option<String>>,
    /// Specifies the destination log group. A log group is created automatically if it doesn't already exist.
    #[builder(into)]
    #[serde(rename = "logGroupName")]
    pub r#log_group_name: Box<String>,
    /// Specifies the pattern for identifying the start of a log message.
    #[builder(into, default)]
    #[serde(rename = "multilineStartPattern")]
    pub r#multiline_start_pattern: Box<Option<String>>,
    /// Specifies the time zone of log event time stamps.
    #[builder(into, default)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Box<Option<String>>,
}
