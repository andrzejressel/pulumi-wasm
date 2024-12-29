#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsVideoDescriptionCodecSettingsH264SettingsFilterSettingsTemporalFilterSettings {
    /// Post filter sharpening.
    #[builder(into, default)]
    #[serde(rename = "postFilterSharpening")]
    pub r#post_filter_sharpening: Box<Option<String>>,
    /// Filter strength.
    #[builder(into, default)]
    #[serde(rename = "strength")]
    pub r#strength: Box<Option<String>>,
}
