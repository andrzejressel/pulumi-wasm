#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CxAgentSpeechToTextSettings {
    /// Whether to use speech adaptation for speech recognition.
    #[builder(into, default)]
    #[serde(rename = "enableSpeechAdaptation")]
    pub r#enable_speech_adaptation: Box<Option<bool>>,
}
