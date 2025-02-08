#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettings {
    #[builder(into, default)]
    #[serde(rename = "audioOnlyHlsSettings")]
    pub r#audio_only_hls_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsAudioOnlyHlsSettings>>,
    #[builder(into, default)]
    #[serde(rename = "fmp4HlsSettings")]
    pub r#fmp_4_hls_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsFmp4HlsSettings>>,
    #[builder(into, default)]
    #[serde(rename = "frameCaptureHlsSettings")]
    pub r#frame_capture_hls_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsFrameCaptureHlsSettings>>,
    #[builder(into, default)]
    #[serde(rename = "standardHlsSettings")]
    pub r#standard_hls_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsStandardHlsSettings>>,
}
