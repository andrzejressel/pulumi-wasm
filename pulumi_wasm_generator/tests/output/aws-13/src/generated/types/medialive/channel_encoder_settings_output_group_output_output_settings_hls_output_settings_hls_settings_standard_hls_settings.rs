#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsStandardHlsSettings {
    #[builder(into, default)]
    #[serde(rename = "audioRenditionSets")]
    pub r#audio_rendition_sets: Box<Option<String>>,
    #[builder(into)]
    #[serde(rename = "m3u8Settings")]
    pub r#m_3_u_8_settings: Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsStandardHlsSettingsM3U8Settings>,
}
