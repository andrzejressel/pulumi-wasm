#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CxPageEventHandler {
    /// The name of the event to handle.
    #[builder(into, default)]
    #[serde(rename = "event")]
    pub r#event: Box<Option<String>>,
    /// (Output)
    /// The unique identifier of this event handler.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The target flow to transition to.
    /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>.
    #[builder(into, default)]
    #[serde(rename = "targetFlow")]
    pub r#target_flow: Box<Option<String>>,
    /// The target page to transition to.
    /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>.
    #[builder(into, default)]
    #[serde(rename = "targetPage")]
    pub r#target_page: Box<Option<String>>,
    /// The fulfillment to call when the event occurs. Handling webhook errors with a fulfillment enabled with webhook could cause infinite loop. It is invalid to specify such fulfillment for a handler handling webhooks.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "triggerFulfillment")]
    pub r#trigger_fulfillment: Box<Option<super::super::types::diagflow::CxPageEventHandlerTriggerFulfillment>>,
}
