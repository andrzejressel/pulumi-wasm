#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct QuickConnectQuickConnectConfigQueueConfig {
    /// Specifies the identifier of the contact flow.
    #[builder(into)]
    #[serde(rename = "contactFlowId")]
    pub r#contact_flow_id: Box<String>,
    /// Specifies the identifier for the queue.
    #[builder(into)]
    #[serde(rename = "queueId")]
    pub r#queue_id: Box<String>,
}
