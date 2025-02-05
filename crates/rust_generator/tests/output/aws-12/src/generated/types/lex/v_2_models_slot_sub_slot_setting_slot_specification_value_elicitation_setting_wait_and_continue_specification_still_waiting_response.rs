#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingWaitAndContinueSpecificationStillWaitingResponse {
    /// Whether the user can interrupt a speech response from Amazon Lex.
    #[builder(into, default)]
    #[serde(rename = "allowInterrupt")]
    pub r#allow_interrupt: Box<Option<bool>>,
    /// How often a message should be sent to the user.
    #[builder(into)]
    #[serde(rename = "frequencyInSeconds")]
    pub r#frequency_in_seconds: Box<i32>,
    #[builder(into, default)]
    #[serde(rename = "messageGroups")]
    pub r#message_groups: Box<Option<Vec<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingWaitAndContinueSpecificationStillWaitingResponseMessageGroup>>>,
    /// If Amazon Lex waits longer than this length of time for a response, it will stop sending messages.
    #[builder(into)]
    #[serde(rename = "timeoutInSeconds")]
    pub r#timeout_in_seconds: Box<i32>,
}
