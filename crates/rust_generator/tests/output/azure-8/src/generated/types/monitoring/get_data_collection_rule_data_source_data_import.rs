#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataCollectionRuleDataSourceDataImport {
    /// An `event_hub_data_source` block as defined below.
    #[builder(into)]
    #[serde(rename = "eventHubDataSources")]
    pub r#event_hub_data_sources: Box<Vec<super::super::types::monitoring::GetDataCollectionRuleDataSourceDataImportEventHubDataSource>>,
}
