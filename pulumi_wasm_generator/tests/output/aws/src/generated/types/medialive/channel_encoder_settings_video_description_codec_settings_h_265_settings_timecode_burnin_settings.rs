#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsTimecodeBurninSettings {
    /// Set a prefix on the burned in timecode.
    #[builder(into, default)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
    /// Sets the size of the burned in timecode.
    #[builder(into, default)]
    #[serde(rename = "timecodeBurninFontSize")]
    pub r#timecode_burnin_font_size: Box<Option<String>>,
    /// Sets the position of the burned in timecode.
    #[builder(into, default)]
    #[serde(rename = "timecodeBurninPosition")]
    pub r#timecode_burnin_position: Box<Option<String>>,
}