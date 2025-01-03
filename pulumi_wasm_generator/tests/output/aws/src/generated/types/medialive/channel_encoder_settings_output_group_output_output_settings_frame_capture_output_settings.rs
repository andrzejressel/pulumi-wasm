#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsFrameCaptureOutputSettings {
    /// String concatenated to the end of the destination filename. Required for multiple outputs of the same type.
    #[builder(into, default)]
    #[serde(rename = "nameModifier")]
    pub r#name_modifier: Box<Option<String>>,
}
