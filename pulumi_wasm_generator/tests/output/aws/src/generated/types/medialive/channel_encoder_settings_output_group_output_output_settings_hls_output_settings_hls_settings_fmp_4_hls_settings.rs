#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsFmp4HlsSettings {
    #[builder(into, default)]
    #[serde(rename = "audioRenditionSets")]
    pub r#audio_rendition_sets: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "nielsenId3Behavior")]
    pub r#nielsen_id_3_behavior: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "timedMetadataBehavior")]
    pub r#timed_metadata_behavior: Box<Option<String>>,
}