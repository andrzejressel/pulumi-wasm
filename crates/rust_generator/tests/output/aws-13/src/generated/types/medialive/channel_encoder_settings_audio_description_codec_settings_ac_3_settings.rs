#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsAudioDescriptionCodecSettingsAc3Settings {
    /// Average bitrate in bits/second.
    #[builder(into, default)]
    #[serde(rename = "bitrate")]
    pub r#bitrate: Box<Option<f64>>,
    /// Specifies the bitstream mode (bsmod) for the emitted AC-3 stream.
    #[builder(into, default)]
    #[serde(rename = "bitstreamMode")]
    pub r#bitstream_mode: Box<Option<String>>,
    /// Dolby Digital coding mode.
    #[builder(into, default)]
    #[serde(rename = "codingMode")]
    pub r#coding_mode: Box<Option<String>>,
    /// Sets the dialnorm of the output.
    #[builder(into, default)]
    #[serde(rename = "dialnorm")]
    pub r#dialnorm: Box<Option<i32>>,
    /// If set to filmStandard, adds dynamic range compression signaling to the output bitstream as defined in the Dolby Digital specification.
    #[builder(into, default)]
    #[serde(rename = "drcProfile")]
    pub r#drc_profile: Box<Option<String>>,
    /// When set to enabled, applies a 120Hz lowpass filter to the LFE channel prior to encoding.
    #[builder(into, default)]
    #[serde(rename = "lfeFilter")]
    pub r#lfe_filter: Box<Option<String>>,
    /// Metadata control.
    #[builder(into, default)]
    #[serde(rename = "metadataControl")]
    pub r#metadata_control: Box<Option<String>>,
}
