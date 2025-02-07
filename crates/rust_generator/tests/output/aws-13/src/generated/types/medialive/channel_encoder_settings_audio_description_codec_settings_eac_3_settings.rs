#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3Settings {
    /// Sets the attenuation control.
    #[builder(into, default)]
    #[serde(rename = "attenuationControl")]
    pub r#attenuation_control: Box<Option<String>>,
    /// Average bitrate in bits/second.
    #[builder(into, default)]
    #[serde(rename = "bitrate")]
    pub r#bitrate: Box<Option<f64>>,
    /// Specifies the bitstream mode (bsmod) for the emitted AC-3 stream.
    #[builder(into, default)]
    #[serde(rename = "bitstreamMode")]
    pub r#bitstream_mode: Box<Option<String>>,
    /// Dolby Digital Plus coding mode.
    #[builder(into, default)]
    #[serde(rename = "codingMode")]
    pub r#coding_mode: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "dcFilter")]
    pub r#dc_filter: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "dialnorm")]
    pub r#dialnorm: Box<Option<i32>>,
    /// Sets the Dolby dynamic range compression profile.
    #[builder(into, default)]
    #[serde(rename = "drcLine")]
    pub r#drc_line: Box<Option<String>>,
    /// Sets the profile for heavy Dolby dynamic range compression.
    #[builder(into, default)]
    #[serde(rename = "drcRf")]
    pub r#drc_rf: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "lfeControl")]
    pub r#lfe_control: Box<Option<String>>,
    /// When set to enabled, applies a 120Hz lowpass filter to the LFE channel prior to encoding.
    #[builder(into, default)]
    #[serde(rename = "lfeFilter")]
    pub r#lfe_filter: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "loRoCenterMixLevel")]
    pub r#lo_ro_center_mix_level: Box<Option<f64>>,
    #[builder(into, default)]
    #[serde(rename = "loRoSurroundMixLevel")]
    pub r#lo_ro_surround_mix_level: Box<Option<f64>>,
    #[builder(into, default)]
    #[serde(rename = "ltRtCenterMixLevel")]
    pub r#lt_rt_center_mix_level: Box<Option<f64>>,
    #[builder(into, default)]
    #[serde(rename = "ltRtSurroundMixLevel")]
    pub r#lt_rt_surround_mix_level: Box<Option<f64>>,
    /// Metadata control.
    #[builder(into, default)]
    #[serde(rename = "metadataControl")]
    pub r#metadata_control: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "passthroughControl")]
    pub r#passthrough_control: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "phaseControl")]
    pub r#phase_control: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "stereoDownmix")]
    pub r#stereo_downmix: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "surroundExMode")]
    pub r#surround_ex_mode: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "surroundMode")]
    pub r#surround_mode: Box<Option<String>>,
}
