#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsCaptionLanguageMapping {
    #[builder(into)]
    #[serde(rename = "captionChannel")]
    pub r#caption_channel: Box<i32>,
    #[builder(into)]
    #[serde(rename = "languageCode")]
    pub r#language_code: Box<String>,
    /// Human readable information to indicate captions available for players (eg. English, or Spanish).
    #[builder(into)]
    #[serde(rename = "languageDescription")]
    pub r#language_description: Box<String>,
}