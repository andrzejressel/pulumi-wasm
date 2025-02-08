#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AlertRuleAnomalyBuiltInRequiredDataConnector {
    /// The ID of the required Data Connector.
    #[builder(into, default)]
    #[serde(rename = "connectorId")]
    pub r#connector_id: Box<Option<String>>,
    /// A list of data types of the required Data Connector.
    #[builder(into, default)]
    #[serde(rename = "dataTypes")]
    pub r#data_types: Box<Option<Vec<String>>>,
}
