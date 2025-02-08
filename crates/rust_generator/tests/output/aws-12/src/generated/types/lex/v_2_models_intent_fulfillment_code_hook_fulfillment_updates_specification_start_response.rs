#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsIntentFulfillmentCodeHookFulfillmentUpdatesSpecificationStartResponse {
    /// Whether the user can interrupt the start message while it is playing.
    #[builder(into, default)]
    #[serde(rename = "allowInterrupt")]
    pub r#allow_interrupt: Box<Option<bool>>,
    /// Delay between when the Lambda fulfillment function starts running and the start message is played. If the Lambda function returns before the delay is over, the start message isn't played.
    #[builder(into, default)]
    #[serde(rename = "delayInSeconds")]
    pub r#delay_in_seconds: Box<Option<i32>>,
    /// Between 1-5 configuration block message groups that contain start messages. Amazon Lex chooses one of the messages to play to the user. See `message_group`.
    #[builder(into, default)]
    #[serde(rename = "messageGroups")]
    pub r#message_groups: Box<Option<Vec<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookFulfillmentUpdatesSpecificationStartResponseMessageGroup>>>,
}
