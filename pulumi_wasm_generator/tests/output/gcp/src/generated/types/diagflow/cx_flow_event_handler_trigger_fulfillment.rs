#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CxFlowEventHandlerTriggerFulfillment {
    /// Conditional cases for this fulfillment.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "conditionalCases")]
    pub r#conditional_cases: Box<Option<Vec<super::super::types::diagflow::CxFlowEventHandlerTriggerFulfillmentConditionalCase>>>,
    /// The list of rich message responses to present to the user.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "messages")]
    pub r#messages: Box<Option<Vec<super::super::types::diagflow::CxFlowEventHandlerTriggerFulfillmentMessage>>>,
    /// Whether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks.
    #[builder(into, default)]
    #[serde(rename = "returnPartialResponses")]
    pub r#return_partial_responses: Box<Option<bool>>,
    /// Set parameter values before executing the webhook.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "setParameterActions")]
    pub r#set_parameter_actions: Box<Option<Vec<super::super::types::diagflow::CxFlowEventHandlerTriggerFulfillmentSetParameterAction>>>,
    /// The tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified.
    #[builder(into, default)]
    #[serde(rename = "tag")]
    pub r#tag: Box<Option<String>>,
    /// The webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>.
    #[builder(into, default)]
    #[serde(rename = "webhook")]
    pub r#webhook: Box<Option<String>>,
}
