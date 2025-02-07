#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CxPageEventHandlerTriggerFulfillmentMessageText {
    /// (Output)
    /// Whether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request.
    #[builder(into, default)]
    #[serde(rename = "allowPlaybackInterruption")]
    pub r#allow_playback_interruption: Box<Option<bool>>,
    /// A collection of text responses.
    #[builder(into, default)]
    #[serde(rename = "texts")]
    pub r#texts: Box<Option<Vec<String>>>,
}
