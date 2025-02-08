#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ChannelEncoderSettingsCaptionDescriptionDestinationSettings {
    /// ARIB Destination Settings.
    #[builder(into, default)]
    #[serde(rename = "aribDestinationSettings")]
    pub r#arib_destination_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsAribDestinationSettings>>,
    /// Burn In Destination Settings. See Burn In Destination Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "burnInDestinationSettings")]
    pub r#burn_in_destination_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsBurnInDestinationSettings>>,
    /// DVB Sub Destination Settings. See DVB Sub Destination Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "dvbSubDestinationSettings")]
    pub r#dvb_sub_destination_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsDvbSubDestinationSettings>>,
    /// EBU TT D Destination Settings. See EBU TT D Destination Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "ebuTtDDestinationSettings")]
    pub r#ebu_tt_d_destination_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEbuTtDDestinationSettings>>,
    /// Embedded Destination Settings.
    #[builder(into, default)]
    #[serde(rename = "embeddedDestinationSettings")]
    pub r#embedded_destination_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEmbeddedDestinationSettings>>,
    /// Embedded Plus SCTE20 Destination Settings.
    #[builder(into, default)]
    #[serde(rename = "embeddedPlusScte20DestinationSettings")]
    pub r#embedded_plus_scte_20_destination_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEmbeddedPlusScte20DestinationSettings>>,
    /// RTMP Caption Info Destination Settings.
    #[builder(into, default)]
    #[serde(rename = "rtmpCaptionInfoDestinationSettings")]
    pub r#rtmp_caption_info_destination_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsRtmpCaptionInfoDestinationSettings>>,
    /// SCTE20 Plus Embedded Destination Settings.
    #[builder(into, default)]
    #[serde(rename = "scte20PlusEmbeddedDestinationSettings")]
    pub r#scte_20_plus_embedded_destination_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsScte20PlusEmbeddedDestinationSettings>>,
    /// SCTE27 Destination Settings.
    #[builder(into, default)]
    #[serde(rename = "scte27DestinationSettings")]
    pub r#scte_27_destination_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsScte27DestinationSettings>>,
    /// SMPTE TT Destination Settings.
    #[builder(into, default)]
    #[serde(rename = "smpteTtDestinationSettings")]
    pub r#smpte_tt_destination_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsSmpteTtDestinationSettings>>,
    /// Teletext Destination Settings.
    #[builder(into, default)]
    #[serde(rename = "teletextDestinationSettings")]
    pub r#teletext_destination_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsTeletextDestinationSettings>>,
    /// TTML Destination Settings. See TTML Destination Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "ttmlDestinationSettings")]
    pub r#ttml_destination_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsTtmlDestinationSettings>>,
    /// WebVTT Destination Settings. See WebVTT Destination Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "webvttDestinationSettings")]
    pub r#webvtt_destination_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsWebvttDestinationSettings>>,
}
