#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IntegrationAccountBatchConfigurationReleaseCriteria {
    /// The batch size in bytes for the Logic App Integration Batch Configuration.
    #[builder(into, default)]
    #[serde(rename = "batchSize")]
    pub r#batch_size: Box<Option<i32>>,
    /// The message count for the Logic App Integration Batch Configuration.
    #[builder(into, default)]
    #[serde(rename = "messageCount")]
    pub r#message_count: Box<Option<i32>>,
    /// A `recurrence` block as documented below.
    #[builder(into, default)]
    #[serde(rename = "recurrence")]
    pub r#recurrence: Box<Option<super::super::types::logicapps::IntegrationAccountBatchConfigurationReleaseCriteriaRecurrence>>,
}
