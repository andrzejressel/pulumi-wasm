#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettings {
    /// MediaLive will perform a failover if the specified audio selector is silent for the specified period. See Audio Silence Failover Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "audioSilenceSettings")]
    pub r#audio_silence_settings: Box<Option<super::super::types::medialive::ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsAudioSilenceSettings>>,
    /// MediaLive will perform a failover if content is not detected in this input for the specified period. See Input Loss Failover Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "inputLossSettings")]
    pub r#input_loss_settings: Box<Option<super::super::types::medialive::ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsInputLossSettings>>,
    /// MediaLive will perform a failover if content is considered black for the specified period. See Video Black Failover Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "videoBlackSettings")]
    pub r#video_black_settings: Box<Option<super::super::types::medialive::ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsVideoBlackSettings>>,
}
