#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetQueueOutboundCallerConfig {
    /// Specifies the caller ID name.
    #[builder(into)]
    #[serde(rename = "outboundCallerIdName")]
    pub r#outbound_caller_id_name: Box<String>,
    /// Specifies the caller ID number.
    #[builder(into)]
    #[serde(rename = "outboundCallerIdNumberId")]
    pub r#outbound_caller_id_number_id: Box<String>,
    /// Outbound whisper flow to be used during an outbound call.
    #[builder(into)]
    #[serde(rename = "outboundFlowId")]
    pub r#outbound_flow_id: Box<String>,
}
