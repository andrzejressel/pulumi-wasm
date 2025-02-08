#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsDvbSubSourceSettings {
    /// If you will configure a WebVTT caption description that references this caption selector, use this field to provide the language to consider when translating the image-based source to text.
    #[builder(into, default)]
    #[serde(rename = "ocrLanguage")]
    pub r#ocr_language: Box<Option<String>>,
    /// When using DVB-Sub with Burn-In or SMPTE-TT, use this PID for the source content. Unused for DVB-Sub passthrough. All DVB-Sub content is passed through, regardless of selectors.
    #[builder(into, default)]
    #[serde(rename = "pid")]
    pub r#pid: Box<Option<i32>>,
}
