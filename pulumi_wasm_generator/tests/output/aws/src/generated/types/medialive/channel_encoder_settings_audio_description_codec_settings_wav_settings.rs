#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsAudioDescriptionCodecSettingsWavSettings {
    #[builder(into, default)]
    #[serde(rename = "bitDepth")]
    pub r#bit_depth: Box<Option<f64>>,
    #[builder(into, default)]
    #[serde(rename = "codingMode")]
    pub r#coding_mode: Box<Option<String>>,
    /// Sample rate in Hz.
    #[builder(into, default)]
    #[serde(rename = "sampleRate")]
    pub r#sample_rate: Box<Option<f64>>,
}