#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataCollectionRuleDataSourcesDataImport {
    /// An `event_hub_data_source` block as defined below.
    #[builder(into)]
    #[serde(rename = "eventHubDataSources")]
    pub r#event_hub_data_sources: Box<Vec<super::super::types::monitoring::DataCollectionRuleDataSourcesDataImportEventHubDataSource>>,
}
