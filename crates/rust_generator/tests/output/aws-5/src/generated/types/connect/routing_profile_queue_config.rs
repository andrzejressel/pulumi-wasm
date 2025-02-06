#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RoutingProfileQueueConfig {
    /// Specifies the channels agents can handle in the Contact Control Panel (CCP) for this routing profile. Valid values are `VOICE`, `CHAT`, `TASK`.
    #[builder(into)]
    #[serde(rename = "channel")]
    pub r#channel: Box<String>,
    /// Specifies the delay, in seconds, that a contact should be in the queue before they are routed to an available agent
    #[builder(into)]
    #[serde(rename = "delay")]
    pub r#delay: Box<i32>,
    /// Specifies the order in which contacts are to be handled for the queue.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
    /// ARN for the queue.
    #[builder(into, default)]
    #[serde(rename = "queueArn")]
    pub r#queue_arn: Box<Option<String>>,
    /// Specifies the identifier for the queue.
    #[builder(into)]
    #[serde(rename = "queueId")]
    pub r#queue_id: Box<String>,
    /// Name for the queue.
    #[builder(into, default)]
    #[serde(rename = "queueName")]
    pub r#queue_name: Box<Option<String>>,
}
