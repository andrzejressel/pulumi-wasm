#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsIntentFulfillmentCodeHook {
    /// Whether the fulfillment code hook is used. When active is false, the code hook doesn't run.
    #[builder(into, default)]
    #[serde(rename = "active")]
    pub r#active: Box<Option<bool>>,
    /// Whether a Lambda function should be invoked to fulfill a specific intent.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Configuration block for settings for update messages sent to the user for long-running Lambda fulfillment functions. Fulfillment updates can be used only with streaming conversations. See `fulfillment_updates_specification`.
    #[builder(into, default)]
    #[serde(rename = "fulfillmentUpdatesSpecification")]
    pub r#fulfillment_updates_specification: Box<Option<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookFulfillmentUpdatesSpecification>>,
    /// Configuration block for settings for messages sent to the user for after the Lambda fulfillment function completes. Post-fulfillment messages can be sent for both streaming and non-streaming conversations. See `post_fulfillment_status_specification`.
    #[builder(into, default)]
    #[serde(rename = "postFulfillmentStatusSpecification")]
    pub r#post_fulfillment_status_specification: Box<Option<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecification>>,
}
