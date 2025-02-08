#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DataCollectionRuleDataSourcesLogFile {
    /// Specifies a list of file patterns where the log files are located. For example, `C:\\JavaLogs\\*.log`.
    #[builder(into)]
    #[serde(rename = "filePatterns")]
    pub r#file_patterns: Box<Vec<String>>,
    /// The data format of the log files. possible value is `text`.
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: Box<String>,
    /// The name which should be used for this data source. This name should be unique across all data sources regardless of type within the Data Collection Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A `settings` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "settings")]
    pub r#settings: Box<Option<super::super::types::monitoring::DataCollectionRuleDataSourcesLogFileSettings>>,
    /// Specifies a list of streams that this data source will be sent to. A stream indicates what schema will be used for this data and usually what table in Log Analytics the data will be sent to. Possible value should be custom stream names.
    #[builder(into)]
    #[serde(rename = "streams")]
    pub r#streams: Box<Vec<String>>,
}
