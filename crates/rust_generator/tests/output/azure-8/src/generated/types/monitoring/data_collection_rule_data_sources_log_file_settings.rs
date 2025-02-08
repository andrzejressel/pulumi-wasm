#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DataCollectionRuleDataSourcesLogFileSettings {
    /// A `text` block as defined below.
    #[builder(into)]
    #[serde(rename = "text")]
    pub r#text: Box<super::super::types::monitoring::DataCollectionRuleDataSourcesLogFileSettingsText>,
}
