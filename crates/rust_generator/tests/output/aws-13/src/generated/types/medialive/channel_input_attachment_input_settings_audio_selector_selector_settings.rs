#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettings {
    /// Audio HLS Rendition Selection. See Audio HLS Rendition Selection for more details.
    #[builder(into, default)]
    #[serde(rename = "audioHlsRenditionSelection")]
    pub r#audio_hls_rendition_selection: Box<Option<super::super::types::medialive::ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioHlsRenditionSelection>>,
    /// Audio Language Selection. See Audio Language Selection for more details.
    #[builder(into, default)]
    #[serde(rename = "audioLanguageSelection")]
    pub r#audio_language_selection: Box<Option<super::super::types::medialive::ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioLanguageSelection>>,
    /// Audio Pid Selection. See Audio PID Selection for more details.
    #[builder(into, default)]
    #[serde(rename = "audioPidSelection")]
    pub r#audio_pid_selection: Box<Option<super::super::types::medialive::ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioPidSelection>>,
    /// Audio Track Selection. See Audio Track Selection for more details.
    #[builder(into, default)]
    #[serde(rename = "audioTrackSelection")]
    pub r#audio_track_selection: Box<Option<super::super::types::medialive::ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelection>>,
}
