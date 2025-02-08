#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsAudioSilenceSettings {
    #[builder(into)]
    #[serde(rename = "audioSelectorName")]
    pub r#audio_selector_name: Box<String>,
    /// The amount of time (in milliseconds) that the active input must be silent before automatic input failover occurs. Silence is defined as audio loss or audio quieter than -50 dBFS.
    #[builder(into, default)]
    #[serde(rename = "audioSilenceThresholdMsec")]
    pub r#audio_silence_threshold_msec: Box<Option<i32>>,
}
