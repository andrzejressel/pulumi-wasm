#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataCollectionRuleDataSourceExtension {
    /// A JSON String which specifies the extension setting.
    #[builder(into)]
    #[serde(rename = "extensionJson")]
    pub r#extension_json: Box<String>,
    /// The name of the VM extension.
    #[builder(into)]
    #[serde(rename = "extensionName")]
    pub r#extension_name: Box<String>,
    /// Specifies a list of data sources this extension needs data from. An item should be a name of a supported data source which produces only one stream. Supported data sources type: `performance_counter`, `windows_event_log`,and `syslog`.
    #[builder(into)]
    #[serde(rename = "inputDataSources")]
    pub r#input_data_sources: Box<Vec<String>>,
    /// Specifies the name of the Data Collection Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies a list of streams that this data source will be sent to. A stream indicates what schema will be used for this data and usually what table in Log Analytics the data will be sent to.
    #[builder(into)]
    #[serde(rename = "streams")]
    pub r#streams: Box<Vec<String>>,
}
