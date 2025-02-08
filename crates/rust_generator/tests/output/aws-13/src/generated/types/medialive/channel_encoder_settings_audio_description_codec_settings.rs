#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ChannelEncoderSettingsAudioDescriptionCodecSettings {
    /// Aac Settings. See AAC Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "aacSettings")]
    pub r#aac_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionCodecSettingsAacSettings>>,
    /// Ac3 Settings. See AC3 Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "ac3Settings")]
    pub r#ac_3_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionCodecSettingsAc3Settings>>,
    /// Eac3 Atmos Settings. See EAC3 Atmos Settings
    #[builder(into, default)]
    #[serde(rename = "eac3AtmosSettings")]
    pub r#eac_3_atmos_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3AtmosSettings>>,
    /// Eac3 Settings. See EAC3 Settings
    #[builder(into, default)]
    #[serde(rename = "eac3Settings")]
    pub r#eac_3_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3Settings>>,
    #[builder(into, default)]
    #[serde(rename = "mp2Settings")]
    pub r#mp_2_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionCodecSettingsMp2Settings>>,
    #[builder(into, default)]
    #[serde(rename = "passThroughSettings")]
    pub r#pass_through_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionCodecSettingsPassThroughSettings>>,
    #[builder(into, default)]
    #[serde(rename = "wavSettings")]
    pub r#wav_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionCodecSettingsWavSettings>>,
}
