#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsMsSmoothOutputSettings {
    #[builder(into, default)]
    #[serde(rename = "h265PackagingType")]
    pub r#h_265_packaging_type: Box<Option<String>>,
    /// String concatenated to the end of the destination filename. Required for multiple outputs of the same type.
    #[builder(into, default)]
    #[serde(rename = "nameModifier")]
    pub r#name_modifier: Box<Option<String>>,
}