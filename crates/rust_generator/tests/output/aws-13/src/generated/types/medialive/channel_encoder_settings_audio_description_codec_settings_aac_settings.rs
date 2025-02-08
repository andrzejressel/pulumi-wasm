#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ChannelEncoderSettingsAudioDescriptionCodecSettingsAacSettings {
    /// Average bitrate in bits/second.
    #[builder(into, default)]
    #[serde(rename = "bitrate")]
    pub r#bitrate: Box<Option<f64>>,
    /// Mono, Stereo, or 5.1 channel layout.
    #[builder(into, default)]
    #[serde(rename = "codingMode")]
    pub r#coding_mode: Box<Option<String>>,
    /// Set to "broadcasterMixedAd" when input contains pre-mixed main audio + AD (narration) as a stereo pair.
    #[builder(into, default)]
    #[serde(rename = "inputType")]
    pub r#input_type: Box<Option<String>>,
    /// AAC profile.
    #[builder(into, default)]
    #[serde(rename = "profile")]
    pub r#profile: Box<Option<String>>,
    /// The rate control mode.
    #[builder(into, default)]
    #[serde(rename = "rateControlMode")]
    pub r#rate_control_mode: Box<Option<String>>,
    /// Sets LATM/LOAS AAC output for raw containers.
    #[builder(into, default)]
    #[serde(rename = "rawFormat")]
    pub r#raw_format: Box<Option<String>>,
    /// Sample rate in Hz.
    #[builder(into, default)]
    #[serde(rename = "sampleRate")]
    pub r#sample_rate: Box<Option<f64>>,
    /// Use MPEG-2 AAC audio instead of MPEG-4 AAC audio for raw or MPEG-2 Transport Stream containers.
    #[builder(into, default)]
    #[serde(rename = "spec")]
    pub r#spec: Box<Option<String>>,
    /// VBR Quality Level - Only used if rateControlMode is VBR.
    #[builder(into, default)]
    #[serde(rename = "vbrQuality")]
    pub r#vbr_quality: Box<Option<String>>,
}
