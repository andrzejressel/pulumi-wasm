#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsIntentFulfillmentCodeHookFulfillmentUpdatesSpecificationUpdateResponseMessageGroupMessageImageResponseCardButton {
    /// Text that appears on the button. Use this to tell the user what value is returned when they choose this button.
    #[builder(into)]
    #[serde(rename = "text")]
    pub r#text: Box<String>,
    /// Value returned to Amazon Lex when the user chooses this button. This must be one of the slot values configured for the slot.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
