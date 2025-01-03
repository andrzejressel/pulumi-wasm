#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsAudioDescription {
    /// Advanced audio normalization settings. See Audio Normalization Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "audioNormalizationSettings")]
    pub r#audio_normalization_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionAudioNormalizationSettings>>,
    /// The name of the audio selector used as the source for this AudioDescription.
    #[builder(into)]
    #[serde(rename = "audioSelectorName")]
    pub r#audio_selector_name: Box<String>,
    /// Applies only if audioTypeControl is useConfigured. The values for audioType are defined in ISO-IEC 13818-1.
    #[builder(into, default)]
    #[serde(rename = "audioType")]
    pub r#audio_type: Box<Option<String>>,
    /// Determined how audio type is determined.
    #[builder(into, default)]
    #[serde(rename = "audioTypeControl")]
    pub r#audio_type_control: Box<Option<String>>,
    /// Settings to configure one or more solutions that insert audio watermarks in the audio encode. See Audio Watermark Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "audioWatermarkSettings")]
    pub r#audio_watermark_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettings>>,
    /// Audio codec settings. See Audio Codec Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "codecSettings")]
    pub r#codec_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionCodecSettings>>,
    #[builder(into, default)]
    #[serde(rename = "languageCode")]
    pub r#language_code: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "languageCodeControl")]
    pub r#language_code_control: Box<Option<String>>,
    /// The name of this audio description.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "remixSettings")]
    pub r#remix_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionRemixSettings>>,
    /// Stream name RTMP destinations (URLs of type rtmp://)
    #[builder(into, default)]
    #[serde(rename = "streamName")]
    pub r#stream_name: Box<Option<String>>,
}
