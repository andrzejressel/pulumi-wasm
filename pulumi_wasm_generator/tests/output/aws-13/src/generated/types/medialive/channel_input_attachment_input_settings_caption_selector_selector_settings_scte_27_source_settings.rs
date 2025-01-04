#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsScte27SourceSettings {
    #[builder(into, default)]
    #[serde(rename = "ocrLanguage")]
    pub r#ocr_language: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "pid")]
    pub r#pid: Box<Option<i32>>,
}
