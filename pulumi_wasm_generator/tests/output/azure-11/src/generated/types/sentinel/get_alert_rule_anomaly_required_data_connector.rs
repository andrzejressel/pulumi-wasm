#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAlertRuleAnomalyRequiredDataConnector {
    /// The ID of the required Data Connector.
    #[builder(into)]
    #[serde(rename = "connectorId")]
    pub r#connector_id: Box<String>,
    /// A list of data types of the required Data Connector.
    #[builder(into)]
    #[serde(rename = "dataTypes")]
    pub r#data_types: Box<Vec<String>>,
}
