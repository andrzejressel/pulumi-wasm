#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetDataCollectionRuleDataSourceDataImportEventHubDataSource {
    /// The Event Hub consumer group name.
    #[builder(into)]
    #[serde(rename = "consumerGroup")]
    pub r#consumer_group: Box<String>,
    /// Specifies the name of the Data Collection Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The stream to collect from Event Hub. Possible value should be a custom stream name.
    #[builder(into)]
    #[serde(rename = "stream")]
    pub r#stream: Box<String>,
}
