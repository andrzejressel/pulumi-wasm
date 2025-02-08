#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3AtmosSettings {
    /// Average bitrate in bits/second.
    #[builder(into, default)]
    #[serde(rename = "bitrate")]
    pub r#bitrate: Box<Option<f64>>,
    /// Dolby Digital Plus with Dolby Atmos coding mode.
    #[builder(into, default)]
    #[serde(rename = "codingMode")]
    pub r#coding_mode: Box<Option<String>>,
    /// Sets the dialnorm for the output.
    #[builder(into, default)]
    #[serde(rename = "dialnorm")]
    pub r#dialnorm: Box<Option<f64>>,
    /// Sets the Dolby dynamic range compression profile.
    #[builder(into, default)]
    #[serde(rename = "drcLine")]
    pub r#drc_line: Box<Option<String>>,
    /// Sets the profile for heavy Dolby dynamic range compression.
    #[builder(into, default)]
    #[serde(rename = "drcRf")]
    pub r#drc_rf: Box<Option<String>>,
    /// Height dimensional trim.
    #[builder(into, default)]
    #[serde(rename = "heightTrim")]
    pub r#height_trim: Box<Option<f64>>,
    /// Surround dimensional trim.
    #[builder(into, default)]
    #[serde(rename = "surroundTrim")]
    pub r#surround_trim: Box<Option<f64>>,
}
