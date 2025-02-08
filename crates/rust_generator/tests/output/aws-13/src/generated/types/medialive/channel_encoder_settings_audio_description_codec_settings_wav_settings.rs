#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
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
