#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CxPageEventHandlerTriggerFulfillmentMessageOutputAudioText {
    /// (Output)
    /// Whether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request.
    #[builder(into, default)]
    #[serde(rename = "allowPlaybackInterruption")]
    pub r#allow_playback_interruption: Box<Option<bool>>,
    /// The SSML text to be synthesized. For more information, see SSML.
    #[builder(into, default)]
    #[serde(rename = "ssml")]
    pub r#ssml: Box<Option<String>>,
    /// The raw text to be synthesized.
    #[builder(into, default)]
    #[serde(rename = "text")]
    pub r#text: Box<Option<String>>,
}
