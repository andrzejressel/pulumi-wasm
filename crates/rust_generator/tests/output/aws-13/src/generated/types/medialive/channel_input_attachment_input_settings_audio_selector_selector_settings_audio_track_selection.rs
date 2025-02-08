#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelection {
    /// Configure decoding options for Dolby E streams - these should be Dolby E frames carried in PCM streams tagged with SMPTE-337. See Dolby E Decode for more details.
    #[builder(into, default)]
    #[serde(rename = "dolbyEDecode")]
    pub r#dolby_e_decode: Box<Option<super::super::types::medialive::ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelectionDolbyEDecode>>,
    /// Selects one or more unique audio tracks from within a source. See Audio Tracks for more details.
    #[builder(into)]
    #[serde(rename = "tracks")]
    pub r#tracks: Box<Vec<super::super::types::medialive::ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelectionTrack>>,
}
