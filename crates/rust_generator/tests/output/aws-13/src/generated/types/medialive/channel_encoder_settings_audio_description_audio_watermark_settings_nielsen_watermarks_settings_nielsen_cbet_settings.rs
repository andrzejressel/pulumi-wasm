#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettingsNielsenCbetSettings {
    #[builder(into)]
    #[serde(rename = "cbetCheckDigitString")]
    pub r#cbet_check_digit_string: Box<String>,
    /// Determines the method of CBET insertion mode when prior encoding is detected on the same layer.
    #[builder(into)]
    #[serde(rename = "cbetStepaside")]
    pub r#cbet_stepaside: Box<String>,
    /// CBET source ID to use in the watermark.
    #[builder(into)]
    #[serde(rename = "csid")]
    pub r#csid: Box<String>,
}
