#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SubscriptionRuleCorrelationFilter {
    /// Content type of the message.
    #[builder(into, default)]
    #[serde(rename = "contentType")]
    pub r#content_type: Box<Option<String>>,
    /// Identifier of the correlation.
    #[builder(into, default)]
    #[serde(rename = "correlationId")]
    pub r#correlation_id: Box<Option<String>>,
    /// Application specific label.
    #[builder(into, default)]
    #[serde(rename = "label")]
    pub r#label: Box<Option<String>>,
    /// Identifier of the message.
    #[builder(into, default)]
    #[serde(rename = "messageId")]
    pub r#message_id: Box<Option<String>>,
    /// A list of user defined properties to be included in the filter. Specified as a map of name/value pairs.
    /// 
    /// > **NOTE:** When creating a subscription rule of type `CorrelationFilter` at least one property must be set in the `correlation_filter` block.
    #[builder(into, default)]
    #[serde(rename = "properties")]
    pub r#properties: Box<Option<std::collections::HashMap<String, String>>>,
    /// Address of the queue to reply to.
    #[builder(into, default)]
    #[serde(rename = "replyTo")]
    pub r#reply_to: Box<Option<String>>,
    /// Session identifier to reply to.
    #[builder(into, default)]
    #[serde(rename = "replyToSessionId")]
    pub r#reply_to_session_id: Box<Option<String>>,
    /// Session identifier.
    #[builder(into, default)]
    #[serde(rename = "sessionId")]
    pub r#session_id: Box<Option<String>>,
    /// Address to send to.
    #[builder(into, default)]
    #[serde(rename = "to")]
    pub r#to: Box<Option<String>>,
}
