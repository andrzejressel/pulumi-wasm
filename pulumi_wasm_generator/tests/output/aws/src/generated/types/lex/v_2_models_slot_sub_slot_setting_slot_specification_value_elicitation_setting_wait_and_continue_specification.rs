#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingWaitAndContinueSpecification {
    /// Specifies whether the bot will wait for a user to respond.
    /// When this field is `false`, wait and continue responses for a slot aren't used.
    /// If the active field isn't specified, the default is `true`.
    #[builder(into, default)]
    #[serde(rename = "active")]
    pub r#active: Box<Option<bool>>,
    /// Response that Amazon Lex sends to indicate that the bot is ready to continue the conversation.
    /// See the `continue_response` argument reference below.
    #[builder(into, default)]
    #[serde(rename = "continueResponses")]
    pub r#continue_responses: Box<Option<Vec<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingWaitAndContinueSpecificationContinueResponse>>>,
    /// Response that Amazon Lex sends periodically to the user to indicate that the bot is still waiting for input from the user.
    /// See the `still_waiting_response` argument reference below.
    #[builder(into, default)]
    #[serde(rename = "stillWaitingResponses")]
    pub r#still_waiting_responses: Box<Option<Vec<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingWaitAndContinueSpecificationStillWaitingResponse>>>,
    /// Response that Amazon Lex sends to indicate that the bot is waiting for the conversation to continue.
    /// See the `waiting_response` argument reference below.
    #[builder(into, default)]
    #[serde(rename = "waitingResponses")]
    pub r#waiting_responses: Box<Option<Vec<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingWaitAndContinueSpecificationWaitingResponse>>>,
}