#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsAudioDescriptionAudioNormalizationSettings {
    /// Audio normalization algorithm to use. itu17701 conforms to the CALM Act specification, itu17702 to the EBU R-128 specification.
    #[builder(into, default)]
    #[serde(rename = "algorithm")]
    pub r#algorithm: Box<Option<String>>,
    /// Algorithm control for the audio description.
    #[builder(into, default)]
    #[serde(rename = "algorithmControl")]
    pub r#algorithm_control: Box<Option<String>>,
    /// Target LKFS (loudness) to adjust volume to.
    #[builder(into, default)]
    #[serde(rename = "targetLkfs")]
    pub r#target_lkfs: Box<Option<f64>>,
}
