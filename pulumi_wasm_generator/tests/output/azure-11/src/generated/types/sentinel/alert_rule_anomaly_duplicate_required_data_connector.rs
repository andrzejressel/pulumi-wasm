#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AlertRuleAnomalyDuplicateRequiredDataConnector {
    /// The ID of the required Data Connector.
    #[builder(into, default)]
    #[serde(rename = "connectorId")]
    pub r#connector_id: Box<Option<String>>,
    /// A list of data types of the required Data Connector.
    #[builder(into, default)]
    #[serde(rename = "dataTypes")]
    pub r#data_types: Box<Option<Vec<String>>>,
}
