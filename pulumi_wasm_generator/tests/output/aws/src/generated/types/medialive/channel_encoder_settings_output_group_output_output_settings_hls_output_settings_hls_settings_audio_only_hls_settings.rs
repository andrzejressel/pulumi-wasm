#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsAudioOnlyHlsSettings {
    #[builder(into, default)]
    #[serde(rename = "audioGroupId")]
    pub r#audio_group_id: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "audioOnlyImage")]
    pub r#audio_only_image: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsAudioOnlyHlsSettingsAudioOnlyImage>>,
    #[builder(into, default)]
    #[serde(rename = "audioTrackType")]
    pub r#audio_track_type: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "segmentType")]
    pub r#segment_type: Box<Option<String>>,
}
