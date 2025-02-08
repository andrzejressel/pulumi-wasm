#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioLanguageSelection {
    /// Selects a specific three-letter language code from within an audio source.
    #[builder(into)]
    #[serde(rename = "languageCode")]
    pub r#language_code: Box<String>,
    /// When set to “strict”, the transport stream demux strictly identifies audio streams by their language descriptor. If a PMT update occurs such that an audio stream matching the initially selected language is no longer present then mute will be encoded until the language returns. If “loose”, then on a PMT update the demux will choose another audio stream in the program with the same stream type if it can’t find one with the same language.
    #[builder(into, default)]
    #[serde(rename = "languageSelectionPolicy")]
    pub r#language_selection_policy: Box<Option<String>>,
}
