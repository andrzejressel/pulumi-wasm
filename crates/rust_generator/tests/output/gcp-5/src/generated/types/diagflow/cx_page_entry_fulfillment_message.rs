#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CxPageEntryFulfillmentMessage {
    /// The channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned.
    #[builder(into, default)]
    #[serde(rename = "channel")]
    pub r#channel: Box<Option<String>>,
    /// Indicates that the conversation succeeded, i.e., the bot handled the issue that the customer talked to it about.
    /// Dialogflow only uses this to determine which conversations should be counted as successful and doesn't process the metadata in this message in any way. Note that Dialogflow also considers conversations that get to the conversation end page as successful even if they don't return ConversationSuccess.
    /// You may set this, for example:
    /// * In the entryFulfillment of a Page if entering the page indicates that the conversation succeeded.
    /// * In a webhook response when you determine that you handled the customer issue.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "conversationSuccess")]
    pub r#conversation_success: Box<Option<super::super::types::diagflow::CxPageEntryFulfillmentMessageConversationSuccess>>,
    /// Indicates that the conversation should be handed off to a live agent.
    /// Dialogflow only uses this to determine which conversations were handed off to a human agent for measurement purposes. What else to do with this signal is up to you and your handoff procedures.
    /// You may set this, for example:
    /// * In the entryFulfillment of a Page if entering the page indicates something went extremely wrong in the conversation.
    /// * In a webhook response when you determine that the customer issue can only be handled by a human.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "liveAgentHandoff")]
    pub r#live_agent_handoff: Box<Option<super::super::types::diagflow::CxPageEntryFulfillmentMessageLiveAgentHandoff>>,
    /// A text or ssml response that is preferentially used for TTS output audio synthesis, as described in the comment on the ResponseMessage message.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "outputAudioText")]
    pub r#output_audio_text: Box<Option<super::super::types::diagflow::CxPageEntryFulfillmentMessageOutputAudioText>>,
    /// A custom, platform-specific payload.
    #[builder(into, default)]
    #[serde(rename = "payload")]
    pub r#payload: Box<Option<String>>,
    /// Specifies an audio clip to be played by the client as part of the response.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "playAudio")]
    pub r#play_audio: Box<Option<super::super::types::diagflow::CxPageEntryFulfillmentMessagePlayAudio>>,
    /// Represents the signal that telles the client to transfer the phone call connected to the agent to a third-party endpoint.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "telephonyTransferCall")]
    pub r#telephony_transfer_call: Box<Option<super::super::types::diagflow::CxPageEntryFulfillmentMessageTelephonyTransferCall>>,
    /// The text response message.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "text")]
    pub r#text: Box<Option<super::super::types::diagflow::CxPageEntryFulfillmentMessageText>>,
}
