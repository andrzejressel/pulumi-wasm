#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct QueueOutboundCallerConfig {
    /// Specifies the caller ID name.
    #[builder(into, default)]
    #[serde(rename = "outboundCallerIdName")]
    pub r#outbound_caller_id_name: Box<Option<String>>,
    /// Specifies the caller ID number.
    #[builder(into, default)]
    #[serde(rename = "outboundCallerIdNumberId")]
    pub r#outbound_caller_id_number_id: Box<Option<String>>,
    /// Specifies outbound whisper flow to be used during an outbound call.
    #[builder(into, default)]
    #[serde(rename = "outboundFlowId")]
    pub r#outbound_flow_id: Box<Option<String>>,
}
