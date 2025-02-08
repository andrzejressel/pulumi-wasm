#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsIntentFulfillmentCodeHookFulfillmentUpdatesSpecification {
    /// Whether fulfillment updates are sent to the user. When this field is true, updates are sent. If the active field is set to true, the `start_response`, `update_response`, and `timeout_in_seconds` fields are required.
    #[builder(into)]
    #[serde(rename = "active")]
    pub r#active: Box<bool>,
    /// Configuration block for the message sent to users when the fulfillment Lambda functions starts running.
    #[builder(into, default)]
    #[serde(rename = "startResponse")]
    pub r#start_response: Box<Option<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookFulfillmentUpdatesSpecificationStartResponse>>,
    /// Length of time that the fulfillment Lambda function should run before it times out.
    #[builder(into, default)]
    #[serde(rename = "timeoutInSeconds")]
    pub r#timeout_in_seconds: Box<Option<i32>>,
    /// Configuration block for messages sent periodically to the user while the fulfillment Lambda function is running.
    #[builder(into, default)]
    #[serde(rename = "updateResponse")]
    pub r#update_response: Box<Option<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookFulfillmentUpdatesSpecificationUpdateResponse>>,
}
