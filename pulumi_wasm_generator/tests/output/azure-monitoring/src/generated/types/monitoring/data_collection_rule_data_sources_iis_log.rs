#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataCollectionRuleDataSourcesIisLog {
    /// Specifies a list of absolute paths where the log files are located.
    #[builder(into, default)]
    #[serde(rename = "logDirectories")]
    pub r#log_directories: Box<Option<Vec<String>>>,
    /// The name which should be used for this data source. This name should be unique across all data sources regardless of type within the Data Collection Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies a list of streams that this data source will be sent to. A stream indicates what schema will be used for this data and usually what table in Log Analytics the data will be sent to. Possible value is `Microsoft-W3CIISLog`.
    #[builder(into)]
    #[serde(rename = "streams")]
    pub r#streams: Box<Vec<String>>,
}