#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettings {
    /// Ancillary Source Settings. See Ancillary Source Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "ancillarySourceSettings")]
    pub r#ancillary_source_settings: Box<Option<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsAncillarySourceSettings>>,
    /// ARIB Source Settings.
    #[builder(into, default)]
    #[serde(rename = "aribSourceSettings")]
    pub r#arib_source_settings: Box<Option<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsAribSourceSettings>>,
    /// DVB Sub Source Settings. See DVB Sub Source Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "dvbSubSourceSettings")]
    pub r#dvb_sub_source_settings: Box<Option<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsDvbSubSourceSettings>>,
    /// Embedded Source Settings. See Embedded Source Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "embeddedSourceSettings")]
    pub r#embedded_source_settings: Box<Option<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsEmbeddedSourceSettings>>,
    /// SCTE20 Source Settings. See SCTE 20 Source Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "scte20SourceSettings")]
    pub r#scte_20_source_settings: Box<Option<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsScte20SourceSettings>>,
    /// SCTE27 Source Settings. See SCTE 27 Source Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "scte27SourceSettings")]
    pub r#scte_27_source_settings: Box<Option<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsScte27SourceSettings>>,
    /// Teletext Source Settings. See Teletext Source Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "teletextSourceSettings")]
    pub r#teletext_source_settings: Box<Option<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsTeletextSourceSettings>>,
}
