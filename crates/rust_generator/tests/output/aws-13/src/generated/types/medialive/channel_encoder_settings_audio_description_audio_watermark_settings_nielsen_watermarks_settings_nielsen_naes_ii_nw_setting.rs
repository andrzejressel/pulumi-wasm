#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettingsNielsenNaesIiNwSetting {
    #[builder(into)]
    #[serde(rename = "checkDigitString")]
    pub r#check_digit_string: Box<String>,
    /// The Nielsen Source ID to include in the watermark.
    #[builder(into)]
    #[serde(rename = "sid")]
    pub r#sid: Box<f64>,
}
